use axum::{
    http::header::{HeaderMap},
};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds_option;

use crate::{
    common::{PgPool, RedisPool},
    auth::check::{in_check},
    util::date_config::date_format,
};


#[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq)]
pub struct AuToken {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub status: Vec<String>,
}
#[derive(Clone, Debug)]
pub struct AuthRedis {
    pub pool: PgPool,
    pub conn: RedisPool
}
impl AuthRedis {
    pub async fn ctx(
        &self, headers: HeaderMap
    ) -> Result<Option<AuToken>, Option<String>> {
        let conn = self.conn.clone();
        match in_check(conn, headers).await {
            Ok(expr) => Ok(expr),
            Err(_) => Ok(None),
        }
    }
}


#[derive(Serialize)]
pub struct UpdateUser {
    pub email: String,
    pub username: String,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize)]
pub struct FormUpdateUser {
    pub email: String,
    pub username: String,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct StrErr {
    pub err: String,
}

#[derive(Serialize)]
pub struct VeriUser {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub status: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct FormLogin {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListUser {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub img: Option<String>,
    #[serde(with = "date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize)]
pub struct FormNewUser {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq)]
pub struct KeyEmail {
    pub key:   String,
    pub email: String,
}

#[derive(Deserialize, Serialize)]
pub struct FormPasswordChange {
    pub password: String,
}