use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationData {
    pub login_url: String,
    pub state: String,
}
