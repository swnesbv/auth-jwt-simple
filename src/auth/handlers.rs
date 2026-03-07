use sqlx::postgres::PgPool;
use axum::{
	extract::{State},
	http::{header::{HeaderMap}},
	response::{Html, IntoResponse},
	Extension,
};
use tera::Context;

use crate::{
	common::{Templates},
	auth::views::{all},
	auth::check::{in_check},
	auth::models::{AuToken},
};


#[axum::debug_handler]
pub async fn index(
	i: Option<AuToken>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {
	let mut context = Context::new();
	match i {
		Some(ref expr) => expr,
        None => {
        	context.insert("i_no", "Caramba bullfighting and damn it");
        	return Err(Html(templates.render("index", &context).unwrap()))
    	}
    };
	context.insert("i", &i);
	Ok(Html(templates.render("index", &context).unwrap()))
}

#[axum::debug_handler]
pub async fn users(
    i: Option<AuToken>,
    headers: HeaderMap,
    State(pool): State<PgPool>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

	if i.is_some() {
		println!(" token i email..! {:?}", i.clone().unwrap().email);
	}
	let visit = in_check(headers.clone()).await;
	match visit {
		Ok(ref expr) => expr,
        Err(Some(err)) => return Err(err.to_string()),
        Err(None) => return Err("None".to_string())
    };

	let mut context = Context::new();
	let all_users = all(pool).await.unwrap();
	context.insert("i", &i);
	context.insert("visit", &visit);
	context.insert("users", &all_users);
	Ok(Html(templates.render("users", &context).unwrap()))
}