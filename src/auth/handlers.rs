use std::sync::Arc;
use axum::{
	extract::{State},
	http::{header::{HeaderMap}},
	response::{Html, IntoResponse},
	Extension,
};
use tera::Context;
use redis::AsyncCommands;

use crate::{
	common::{Templates, DoubleConn},
	auth::views::{all},
	auth::check::{in_check},
	auth::models::{AuToken, AuTRedis},
};

//#[axum::debug_handler(state = (RedisPool, PgPool))]
pub async fn index(
	i: Option<AuToken>,
	State(dc): State<Arc<AuTRedis>>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

	let mut context = Context::new();

    let mut rs = match dc.conn.get().await{
        Ok(expr) => expr,
        Err(err) => {
        	context.insert("err", &err.to_string());
        	return Err(Html(templates.render("index", &context).unwrap()))
        }
    };
	// rs.set::<&str, &str, ()>("key", "value").await.unwrap();
    let k: String = rs.get("key").await.unwrap();

	let email: String = rs.get("email").await.unwrap_or("None..!".to_string());
	println!(" index email..! {:?}", email);
    println!(" index key..! {:?}", k);

    let _ = all(dc.pool.clone()).await.unwrap();

	match i {
		Some(expr) => {
        	context.insert("i", &expr);
        	Ok(Html(templates.render("index", &context).unwrap()))
		},
        None => {
        	context.insert("is_no", "index Err Caramba bullfighting and damn it");
        	Err(Html(templates.render("index", &context).unwrap()))
    	}
    }
}

pub async fn users(
    headers: HeaderMap,
    State(dc): State<Arc<DoubleConn>>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

	let mut context = Context::new();

    let mut rs = match dc.conn.get().await{
        Ok(expr) => expr,
        Err(err) => {
        	context.insert("err", &err.to_string());
        	return Err(Html(templates.render("index", &context).unwrap()))
        }
    };
    let r: String = rs.get("key").await.unwrap_or("None..!".to_string());
    let email: String = rs.get("email").await.unwrap_or("None..!".to_string());
    println!(" users key..! {:?}", r);
    println!(" users email..! {:?}", email);

	let _ = match in_check(headers.clone(), dc.conn.clone()).await {
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
	match all(dc.pool.clone()).await {
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

pub async fn i_users(
    i: Option<AuToken>,
    Extension(templates): Extension<Templates>,
) -> impl IntoResponse {

	let mut context = Context::new();
	context.insert("i", &i);
	Html(templates.render("i_users", &context).unwrap())
}