use crate::{
    core::user::{hash_password, User},
    error::{ApiError, ApiResult},
};
use anyhow::Context;
use http::StatusCode;
use lazy_static::lazy_static;
use rand::RngCore;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::Pool;
use uuid::Uuid;
use validator::{Validate, ValidationError};

lazy_static! {
    static ref RE_USERNAME: Regex = Regex::new(r"^[a-zA-Z0-9_-]{3,20}$").expect("Invalid regex");
    static ref RE_PASS: Regex = Regex::new(r"^*{6,}$").expect("Invalid regex");
}

#[derive(Validate, Serialize, Deserialize)]
#[validate(schema(function = "validate_confirm_password", skip_on_field_errors = false))]
#[serde(rename_all = "camelCase")]
pub struct SignUpCreds {
    #[validate(regex = "RE_USERNAME")]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(regex = "RE_PASS")]
    pub password: String,
    pub confirm_password: String,
}

#[derive(Validate, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginCreds {
    #[validate(custom = "validate_email_or_username")]
    pub username_or_email: String,
    #[validate(regex = "RE_PASS")]
    pub password: String,
}

fn validate_confirm_password(data: &SignUpCreds) -> Result<(), ValidationError> {
    (data.password == data.confirm_password)
        .then(|| ())
        .ok_or_else(|| ValidationError::new("confirm_password"))
}

fn validate_email_or_username(email_usr: &String) -> Result<(), ValidationError> {
    let matches = validator::validate_email(email_usr) || RE_USERNAME.is_match(&email_usr);
    matches
        .then(|| ())
        .ok_or_else(|| ValidationError::new("username_email"))
}

pub async fn register_user(creds: &SignUpCreds, pool: &Pool<sqlx::Postgres>) -> ApiResult<User> {
    if creds.validate().is_err() {
        return Err(ApiError::new(
            "error validating sign up body",
            StatusCode::BAD_REQUEST,
        ));
    }

    let exist_user = sqlx::query!(
        "SELECT id FROM account WHERE name = $1 LIMIT 1",
        creds.username
    )
    .fetch_one(pool)
    .await;

    if exist_user.is_ok() {
        return Err(ApiError::new(
            "invalid credentials",
            StatusCode::UNAUTHORIZED,
        ));
    }

    let mut salt = [0u8; 8];
    rand::thread_rng().fill_bytes(&mut salt);

    let hashed_passwd = hash_password(&creds.password, &salt)?;

    let id = Uuid::new_v4();

    let result = sqlx::query_as!(
        User,
        "INSERT INTO account(id, name, email, password, salt) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        id,
        creds.username,
        creds.email,
        hashed_passwd,
        salt.to_vec()
    )
    .fetch_one(pool)
    .await.with_context(|| "unable to insert user").map_err(|err| err.into());

    result
}

pub async fn login_user(creds: &LoginCreds, pool: &Pool<sqlx::Postgres>) -> ApiResult<User> {
    let exist_user: User = sqlx::query_as!(
        User,
        "SELECT * FROM account WHERE name = $1 OR email = $1 LIMIT 1",
        creds.username_or_email
    )
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::error::Error::RowNotFound => {
            ApiError::new("invalid credentials", StatusCode::UNAUTHORIZED)
        }
        _ => e.into(),
    })?;

    let input_hash = hash_password(&creds.password, &exist_user.salt)?;

    if input_hash == exist_user.password {
        Ok(exist_user)
    } else {
        Err(ApiError::new(
            "invalid credentials",
            StatusCode::UNAUTHORIZED,
        ))
    }
}
