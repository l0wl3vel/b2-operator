use futures::executor::block_on;
use log::{debug, error, info, log_enabled, warn, Level};
use std::{
    collections::{BTreeMap, HashMap},
    sync::{Arc, Mutex},
    time,
};

use b2::{client::HyperClient, Authorization};
use k8s_openapi::{apimachinery::pkg::apis::meta::v1::Time, http::Error};
use kube::{
    api::ListParams,
    error,
    runtime::{controller::Action, Controller},
    Api, Client,
};
use tokio::time::Duration;

use crate::helpers::*;
use crate::{
    crds::{Account, AccountSecretReference},
    helpers::b2::{Credential, TimestampedAuthorization},
};

use b2_client as b2;

type b2_api_client = b2::client::HyperClient;

struct Context {
    k8s_client: Client,
    b2_client: b2::client::HyperClient,
    b2_authorizations: Mutex<HashMap<String, Box<TimestampedAuthorization>>>,
}

pub async fn start_account_controller() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;
    let accounts: Api<Account> = Api::all(client.clone());

    let b2_client = b2_api_client::default();

    futures::StreamExt::for_each(
        Controller::new(accounts, ListParams::default()).run(
            reconcile_account,
            error_policy,
            Arc::new(Context {
                k8s_client: client,
                b2_client,
                b2_authorizations: Mutex::new(HashMap::new()),
            }),
        ),
        |res| async move {
            match res {
                Ok(o) => info!("reconciled {:?}", o),
                Err(e) => warn!("reconcile failed: {}", e),
            }
        },
    )
    .await;
    Ok(())
}

async fn reconcile_account(generator: Arc<Account>, ctx: Arc<Context>) -> Result<Action, Error> {
    info!(
        "Reconsiling Account {}",
        generator.metadata.name.clone().unwrap()
    );
    let b2_client = ctx.b2_client.clone();
    let k8s_client = ctx.k8s_client.clone();

    let b2_credential =
        get_b2_credentials_from_secret(generator.spec.credential_reference.clone(), k8s_client)
            .await
            .unwrap();
    {
        let auth_exists = match ctx.b2_authorizations.lock() {
            Ok(auth) => auth.contains_key(&b2_credential.key_id),
            Err(_) => panic!("Failed to aquire Mutex"),
        };

        if !auth_exists {
            let authorization = b2::authorize_account(
                b2_client,
                &b2_credential.key_id,
                &b2_credential.application_key,
            )
            .await;
            match authorization {
                Ok(a) => {
                    info!("Authorization aquired{:?}", a);
                    let ts_auth = TimestampedAuthorization::new(a);
                    ctx.b2_authorizations
                        .lock()
                        .unwrap()
                        .insert(b2_credential.key_id.clone(), Box::new(ts_auth));
                    Ok(Action::requeue(Duration::from_secs(1)))
                }
                Err(e) => match e {
                    b2::Error::Client(_) => {
                        error!("Client Error, retrying in 30s");
                        Ok(Action::requeue(Duration::from_secs(30)))
                    }
                    b2::Error::B2(e) => match e.code() {
                        b2::error::ErrorCode::Unauthorized => {
                            error!(
                                "Wrong Credentials for account {:?}",
                                generator.metadata.clone()
                            );
                            Ok(Action::requeue(Duration::from_secs(300)))
                        }
                        _ => {
                            error!("Other Error with B2 API detected: {}", e);
                            Ok(Action::requeue(Duration::from_secs(60)))
                        }
                    },
                    _ => {
                        error!("Other Error with B2 API detected: {}", e);
                        Ok(Action::requeue(Duration::from_secs(60)))
                    }
                },
            }
        } else {
            //TODO: Reconsile Account
            Ok(Action::requeue(Duration::from_secs(300)))
        }
    }
}

fn error_policy(_object: Arc<Account>, _error: &Error, _ctx: Arc<Context>) -> Action {
    Action::requeue(Duration::from_secs(1))
}

pub async fn get_b2_credentials_from_secret(
    secret_description: AccountSecretReference,
    k8s_client: Client,
) -> Option<Credential> {
    let secret: Api<k8s_openapi::api::core::v1::Secret> =
        Api::namespaced(k8s_client, &secret_description.namespace);

    let secret_object = secret.get(&secret_description.name).await.unwrap();

    let secret_content = secret::decode(&secret_object);

    let credentials = Credential {
        key_id: secret_content
            .get(&secret_description.key_id_field)
            .unwrap()
            .to_string(),
        application_key: secret_content
            .get(&secret_description.application_key_field)
            .unwrap()
            .to_string(),
    };
    info!("{:?}", credentials);
    Some(credentials)
}
