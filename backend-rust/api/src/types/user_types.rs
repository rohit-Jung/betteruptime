use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserSignInResponse {
    pub id: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserSignupBody {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserSigninBody {
    pub username: String,
    pub password: String,
}
