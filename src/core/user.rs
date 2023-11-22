use argon2::{Config as ArgonConfig, Variant::Argon2id};
use axum_login::secrecy::SecretVec;
use axum_login::AuthUser;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub salt: Vec<u8>,
    pub created_at: Option<DateTime<Utc>>,
}

impl AuthUser<Uuid> for User {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_password_hash(&self) -> SecretVec<u8> {
        SecretVec::new(self.password.clone().into())
    }
}

pub fn hash_password(plain: &str, salt: &[u8]) -> Result<String, argon2::Error> {
    let config = ArgonConfig {
        variant: Argon2id,
        ..ArgonConfig::default()
    };
    argon2::hash_encoded(plain.as_bytes(), &salt, &config)
}
