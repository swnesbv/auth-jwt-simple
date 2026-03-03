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
    headers: HeaderMap,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

	if i.is_some() {
		println!(" token i email..! {:?}", i.clone().unwrap().email);
	}

	let mut context = Context::new();
	let visit = in_check(headers.clone()).await;
	match visit {
		Ok(expr) => {
			context.insert("i", &i);
		    context.insert("token", &expr);
		    Ok(Html(templates.render("index", &context).unwrap()))
		}
        Err(err) => {
            context.insert("err", &err);
            Err(Html(templates.render("index", &context).unwrap()))
        }
    }
}

pub async fn users(
    i: Option<AuToken>,
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

	if i.is_some() {
		println!(" token i email..! {:?}", i.clone().unwrap().email);
	}

	let mut context = Context::new();
	let visit = in_check(headers.clone()).await;
	match visit {
		Ok(expr) => {
			let users = all(pool).await.unwrap();
			context.insert("i", &i);
		    context.insert("token", &expr);
			context.insert("users", &users);
		    Ok(Html(templates.render("users", &context).unwrap()))
		}
        Err(err) => {
            context.insert("err", &err);
            Err(Html(templates.render("users", &context).unwrap()))
        }
    }
}