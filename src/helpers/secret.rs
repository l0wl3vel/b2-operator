use std::collections::BTreeMap;

pub fn decode(secret: &k8s_openapi::api::core::v1::Secret) -> BTreeMap<String, String> {
    let mut res = BTreeMap::new();
    // Ignoring binary data for now
    if let Some(data) = secret.data.clone() {
        for (k, v) in data {
            if let Ok(b) = std::str::from_utf8(&v.0) {
                res.insert(k, b.to_string());
            } else {
                panic!("Secret {} contains binary data!", secret.metadata.name.clone().unwrap());
            }
        }
    }
    res
}