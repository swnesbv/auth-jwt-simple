use axum::{
    extract::{FromRef, FromRequestParts, OptionalFromRequestParts},
    http::{request::Parts},
    http::header::{HeaderMap},
};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds_option;

use crate::{
    util::date_config::date_format,
    auth::check::{in_check},
};


#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct StrErr {
    pub err: String,
}


#[derive(sqlx::FromRow, Debug)]
pub struct AdminStatus {
    pub status: Option<Vec<String>>,
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

#[derive(Clone, Serialize)]
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


#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct AuToken {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub status: Vec<String>,
}

impl<S> OptionalFromRequestParts<S> for AuToken
where
    S: FromRef<S>,
    S: Send + Sync + Clone + 'static,
{
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {

        let headers = HeaderMap::from_request_parts(parts, state)
            .await;
        match headers {
            Ok(expr) => {
                let a = in_check(expr).await;
                match a {
                    Ok(expr) => Ok(expr),
                    Err(_) => Ok(None),
                }
            }
            Err(_) => Ok(None),
        }
    }
}