use axum::{extract::State, response::IntoResponse, Extension, Json};
use axum_login::PostgresStore;
use http::StatusCode;
use serde_json::json;
use uuid::Uuid;

use crate::{config::AppContext, error::ApiError, models::User, services::user};

type AuthContext = axum_login::extractors::AuthContext<Uuid, User, PostgresStore<User>>;

pub async fn register_user(
    mut auth: AuthContext,
    State(state): State<AppContext>,
    Json(input): Json<user::SignUpCreds>,
) -> Result<impl IntoResponse, ApiError> {
    if let Some(user) = auth.current_user {
        return Ok((StatusCode::OK, Json(user)));
    }
    let user = user::register_user(&input, &state.pool).await?;
    auth.login(&user).await.unwrap();
    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn login_user(
    mut auth: AuthContext,
    State(state): State<AppContext>,
    Json(input): Json<user::LoginCreds>,
) -> Result<impl IntoResponse, ApiError> {
    if let Some(user) = auth.current_user {
        return Ok(Json(user));
    }
    let user = user::login_user(&input, &state.pool).await?;
    auth.login(&user).await.unwrap();
    Ok(Json(user))
}

pub async fn logout_user(mut auth: AuthContext) -> Result<impl IntoResponse, ApiError> {
    auth.logout().await;
    Ok(Json(json!({"ok": "true"})))
}

pub async fn me(Extension(user): Extension<User>) -> Result<impl IntoResponse, ApiError> {
    Ok(format!("Hey {}", user.name))
}
