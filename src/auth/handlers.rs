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

pub async fn index(
	i: Option<AuToken>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {
	let mut context = Context::new();
	match i {
		Some(expr) => {
        	context.insert("i", &expr);
        	Ok(Html(templates.render("index", &context).unwrap()))
		},
        None => {
        	context.insert("is_no", "Err Caramba bullfighting and damn it");
        	Err(Html(templates.render("index", &context).unwrap()))
    	}
    }
}

pub async fn i_users(
    i: Option<AuToken>,
    Extension(templates): Extension<Templates>,
) -> impl IntoResponse {

	let mut context = Context::new();
	if i.is_some() {
		context.insert("i", &i);
	}
	if i.is_none() {
    	context.insert("is_no", "None Caramba bullfighting and damn it");
	}
	Html(templates.render("i_users", &context).unwrap())
}

pub async fn users(
    headers: HeaderMap,
    State(pool): State<PgPool>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

	let mut context = Context::new();

	// let v = in_check(headers.clone()).await;

	// if v.clone().ok().is_some() {
    // 	context.insert("visit", &v.clone().ok());
    // 	println!(" is_ok.. {:?}", context)
	// }
	// if v.is_err() {
    // 	context.insert("err", &v.clone().err());
    // 	println!(" is_err.. {:?}", context)
	// }
	// if v.ok().is_none() {
    // 	context.insert(
    // 		"is_no", "Err-None Caramba bullfighting and damn it"
    // 	);
    // 	println!(" is_no.. {:?}", context)
	// }

	let _ = match in_check(headers.clone()).await {
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
	match all(pool).await {
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