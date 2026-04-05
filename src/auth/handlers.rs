use std::sync::Arc;
use axum::{
	extract::{State, Path},
	http::{header::{HeaderMap}},
	response::{Html, IntoResponse},
	Extension,
};
use tera::Context;
use redis::{AsyncCommands, RedisError};

use crate::{
	common::{Templates},
	auth::views::{all, details},
	auth::check::{in_check},
	auth::models::{AuToken, AuthRedis, KeyEmail},
};

// #[axum::debug_handler()]
pub async fn index(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();

    let mut rs = match i.conn.get().await{
        Ok(expr) => expr,
        Err(err) => {
            context.insert("err", &err.to_string());
            return Err(Html(templates.render("index", &context).unwrap()))
        }
    };

    let json_key: Result<Option<String>, RedisError> = rs.get("session").await;
    println!(" index json_key..! {:#?}", json_key);

    let key: Result<Option<String>, RedisError> = rs.get("key").await;
    let email: Result<Option<String>, RedisError> = rs.get("email").await;
    match key {
        Ok(_) => match email {
            Ok(ref expr) => expr,
            Err(err) => {
                context.insert("err", &err.to_string());
                return Err(
                    Html(templates.render("index", &context).unwrap())
                )
            }
        },
        Err(err) => {
            context.insert("err", &err.to_string());
            return Err(Html(templates.render("index", &context).unwrap()))
        }
    };
    println!(" index key..! {:#?}", key);
    println!(" index email..! {:#?}", email);

    let _ = all(i.pool.clone()).await.unwrap();
    //..

    let a = i.ctx(headers).await;

    match a {
        Ok(expr) => {
            println!(" index a..! {:#?}", expr);
            context.insert("i", &expr);
            Ok(Html(templates.render("index", &context).unwrap()))
        },
        Err(Some(err)) => {
            context.insert("err", &err.to_string());
            Err(Html(templates.render("index", &context).unwrap()))
        }
        Err(None) => {
            context.insert("is_no", "index Err Caramba bullfighting and damn it");
            Err(Html(templates.render("index", &context).unwrap()))
        }
    }

}

pub async fn users(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();

    let mut rs = match i.conn.get().await{
        Ok(expr) => expr,
        Err(err) => {
            context.insert("err", &err.to_string());
            return Err(Html(templates.render("users", &context).unwrap()))
        }
    };

    let a: String = rs.get("auth").await.unwrap();
    let b: KeyEmail = serde_json::from_str(&a).unwrap();
    println!(" json j_get..! {:#?}", b.email);

    let key: Option<String> = rs.get("key").await.unwrap_or(Some("None..!".to_string()));
    let email: Option<String> = rs.get("email").await.unwrap_or(Some("None..!".to_string()));
    println!(" redis key..! {:#?}", key);
    println!(" redis email..! {:#?}", email);

    let _ = match in_check(i.conn.clone(), headers.clone()).await {
        Ok(expr) => {
            context.insert("visit", &expr);
            Ok(Html(templates.render("users", &context).unwrap()))
        }
        Err(Some(err)) => {
            context.insert("err", &err);
            Err(Html(templates.render("users", &context).unwrap()))
        }
        Err(None) => {
            context.insert("is_no", "Err-None Caramba bullfighting and damn it");
            Err(Html(templates.render("users", &context).unwrap()))
        }
    };
    match all(i.pool.clone()).await {
        Ok(expr) => {
            context.insert("all_users", &expr);
            Ok(Html(templates.render("users", &context).unwrap()))
        }
        Err(Some(err)) => {
            context.insert("err", &err.to_string());
            Err(Html(templates.render("users", &context).unwrap()))
        }
        Err(None) => {
            context.insert("is_no", "Err-None Caramba bullfighting and damn it");
            Err(Html(templates.render("users", &context).unwrap()))
        }
    }
}


pub async fn user(
    Path(name): Path<String>,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();

    let user = details(i.pool.clone(), name).await;
    match user {
        Ok(expr) => {
            context.insert("user", &expr);
            Ok(Html(templates.render("user", &context).unwrap()))
        }
        Err(Some(err)) => {
            context.insert("err", &err.to_string());
            Err(Html(templates.render("user", &context).unwrap()))
        }
        Err(None) => {
            context.insert("is_no", "Caramba bullfighting and damn it");
            Err(Html(templates.render("user", &context).unwrap()))
        }
    }
}

pub async fn i_users(
    i: Option<AuToken>,
    Extension(templates): Extension<Templates>,
) -> impl IntoResponse {

	let mut context = Context::new();
	context.insert("i", &i);
	Html(templates.render("i_users", &context).unwrap())
}