use b2_client::{Authorization, client::HyperClient};

pub type B2KeyId = String;
pub type B2ApplicationKey = String;

const auth_token_valid_duration: std::time::Duration = std::time::Duration::from_secs(6*3600); // Assume Token is valid for 6 hours

#[derive(Debug)]
pub struct Credential {
    pub key_id: B2KeyId,
    pub application_key: B2KeyId,
}

#[derive(Debug)]
pub struct TimestampedAuthorization   {
    pub authorization: Box<Authorization<HyperClient>>,
    creation_time: std::time::Instant,
}

impl TimestampedAuthorization {
    pub fn new(authorization: Authorization<HyperClient>) -> Self {
        TimestampedAuthorization  {
            authorization: Box::new(authorization),
            creation_time: std::time::Instant::now()
        }
    }
}