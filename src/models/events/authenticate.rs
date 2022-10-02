use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AuthData {
    pub application: Application,
    pub expires: String,
    pub user: User,
    pub scopes: Vec<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Application {
    pub description: String,
    pub icon: String,
    pub id: String,
    pub rpc_origins: Vec<String>,
    pub name: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub discriminator: String,
    pub id: String,
    pub avatar: String,
}
