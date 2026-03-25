use std::sync::{Arc, Mutex};

use bcrypt::verify;
use jsonwebtoken::{encode, EncodingKey, Header};
use poem::{
    handler,
    web::{Data, Json},
};

use store::store::Store;

use crate::types::{
    claims_types::Claims,
    error_types::ApiError,
    user_types::{UserSignInResponse, UserSigninBody, UserSignupBody},
};

#[handler]
pub fn signin_user(
    Json(payload): Json<UserSigninBody>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> poem::Result<Json<UserSignInResponse>> {
    let mut locked_s = s.lock().unwrap();

    let user = locked_s.get_user(payload.username).map_err(|_| ApiError {
        code: 404,
        error: "User not found".to_string(),
    })?;

    let is_password_correct = verify(payload.password, &user.password).map_err(|_| ApiError {
        code: 403,
        error: "Error in checking password".to_string(),
    })?;

    if !is_password_correct {
        return Err(ApiError {
            code: 403,
            error: "Invalid Credentials".to_string(),
        }
        .into());
    }

    let my_claims = Claims {
        sub: String::from("value"),
        exp: 30,
    };

    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .unwrap();

    let response = UserSignInResponse {
        id: String::from("id"),
        token,
    };

    Ok(Json(response))
}

#[handler]
pub fn signup_user(
    Json(payload): Json<UserSignupBody>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Result<Json<UserSignInResponse>> {
    let username = payload.username;
    let password = payload.password;

    // FIX: Throw error here too
    let mut locked_s = s.lock().unwrap();
    let user = locked_s
        .create_user(username, password)
        .map_err(|_| ApiError {
            code: 403,
            error: "Couldn't create user".to_string(),
        })?;



    let response = UserSignInResponse {
        id: String::from("id"),
        token: String::from("token"),
    };

    Ok(Json(response))
}
