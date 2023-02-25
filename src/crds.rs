use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use kube::CustomResource;


#[derive(CustomResource, Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
#[kube(group = "backblaze.b2", version = "v1", kind = "Account")]
pub struct AccountSpec  {
    pub credential_reference: AccountSecretReference
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
pub struct AccountSecretReference {
    pub name: String,
    pub namespace: String,
    pub key_id_field: String,
    pub application_key_field: String,
}


#[derive(CustomResource, Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
#[kube(group = "backblaze.b2", version = "v1", kind = "Bucket")]
pub struct BucketSpec   {
    account_reference: String,
}

#[derive(CustomResource, Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
#[kube(group = "backblaze.b2", version = "v1", kind = "Key")]
pub struct KeySpec   {
    target_secrets: Vec<AccountSecretReference>,
}