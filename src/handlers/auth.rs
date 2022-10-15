use crate::auth::{encode_jwt, hash, PrivateClaim};
use crate::database::PoolType;
use crate::errors::ApiError;
use crate::handlers::user::{UserResponse, self};
use crate::helpers::{respond_json, respond_ok};
use crate::models::user::find_by_auth;
use crate::validate::validate;
use actix_identity::Identity;
use actix_web::{ HttpResponse, web::{block, Data, Json}};
use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "email must be a valid email"))]
    pub email: String,

    #[validate(length(
        min = 6,
        message = "password is required and must be at least 6 characters"
    ))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    access_token: String,
}

impl LoginResponse {
    pub fn new(token: String) -> Self {
        Self { access_token: token}
    }
}

/// Login a user
/// Create and remember their JWT
pub async fn login(
    // id: Identity,
    pool: Data<PoolType>,
    params: Json<LoginRequest>,
) -> Result<Json<LoginResponse>, ApiError> {
    validate(&params)?;

    // Validate that the email + hashed password matches
    let hashed = hash(&params.password);
    let user_resp = block(move || find_by_auth(&pool, &params.email, &hashed)).await?;

    let user = match user_resp {
        Ok(user) => user,
        Err(e) => return Err(e),
    };

    let private_claim = PrivateClaim::new(user.id, user.email.clone());
    let jwt = encode_jwt(private_claim)?;

    respond_json(LoginResponse::new(jwt))
}

/// Logout a user
/// Forget their user_id
pub async fn logout() -> Result<HttpResponse, ApiError> {
    println!("------");
    // println!("{:?}", id.id());
    // id.forget();
    // id.logout();
    respond_ok()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::tests::helpers::tests::get_data_pool;
    use actix_identity::Identity;
    use actix_web::{test, FromRequest};

    async fn get_identity() -> Identity {
        let (request, mut payload) =
            test::TestRequest::with_header("content-type", "application/json").to_http_parts();
        let identity = Option::<Identity>::from_request(&request, &mut payload)
            .await
            .unwrap()
            .unwrap();
        identity
    }

    async fn login_user() -> Result<Json<UserResponse>, ApiError> {
        let params = LoginRequest {
            email: "satoshi@nakamotoinstitute.org".into(),
            password: "123456".into(),
        };
        let identity = get_identity().await;
        login(identity, get_data_pool(), Json(params)).await
    }

    async fn logout_user() -> Result<HttpResponse, ApiError> {
        let identity = get_identity().await;
        logout(identity).await
    }

    #[actix_rt::test]
    async fn it_logs_a_user_in() {
        let response = login_user().await;
        assert!(response.is_ok());
    }

    #[actix_rt::test]
    async fn it_logs_a_user_out() {
        login_user().await.unwrap();
        let response = logout_user().await;
        assert!(response.is_ok());
    }
}
