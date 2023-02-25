use std::future;

use clap::{Parser, Subcommand};
use controllers::account::{self, get_b2_credentials_from_secret, start_account_controller};
use crds::AccountSecretReference;
use kube::{client, core::ObjectMeta, CustomResourceExt};
use tokio::task::spawn_blocking;

mod controllers;
mod crds;
mod helpers;

#[derive(Debug, Parser)]
#[command(name = "b2-operator")]
#[command(about = "K8s Operator for Backblaze B2", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Crds,
    ExampleResources,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    env_logger::init();
    let client = futures::executor::block_on(client::Client::try_default()).unwrap();

    let secret = futures::executor::block_on(get_b2_credentials_from_secret(
        AccountSecretReference {
            name: "test-credentials".to_string(),
            namespace: "default".to_string(),
            key_id_field: "key_id".to_string(),
            application_key_field: "application_key".to_string(),
        }, client));

    match args.command {
        Some(Commands::Crds) => print_crd_definitions(),
        Some(Commands::ExampleResources) => print_example_crs(),
        _ => start_b2_operator().await,
    }
}

async fn start_b2_operator() {
    start_account_controller().await;
}

fn print_crd_definitions() {
    use serde::Serialize;
    let mut buffer = Vec::new();
    let mut ser = serde_yaml::Serializer::new(&mut buffer);

    crds::Account::crd().serialize(&mut ser).unwrap();
    crds::Bucket::crd().serialize(&mut ser).unwrap();
    crds::Key::crd().serialize(&mut ser).unwrap();
    println!("{}", String::from_utf8(buffer).unwrap());
}

fn print_example_crs() {
    use serde::Serialize;
    let mut buffer = Vec::new();
    let mut ser = serde_yaml::Serializer::new(&mut buffer);

    let example_account = crds::Account {
        metadata: ObjectMeta {
            name: Some("test".to_string()),
            ..Default::default()
        },
        spec: crds::AccountSpec {
            credential_reference: crds::AccountSecretReference {
                name: "Yeet".to_owned(),
                namespace: "example_namespace".to_owned(),
                key_id_field: "key_id".to_owned(),
                application_key_field: "application_key".to_owned(),
            },
        },
    };

    example_account.serialize(&mut ser).unwrap();

    println!("{}", String::from_utf8(buffer).unwrap());
}
