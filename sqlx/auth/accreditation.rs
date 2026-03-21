use std::fs;
use std::fs::File;
use std::io::prelude::*;
use sqlx::postgres::PgPool;
use sqlx::{query, query_as};
use axum::{
	body::Body,
	extract::{Form, State},
	http::{Response, StatusCode, header::{HeaderMap}},
	response::{Html, IntoResponse, Redirect},
	Extension,
};
use chrono::Utc;
use rand::distr::{Alphanumeric, SampleString};
use pbkdf2::{Pbkdf2,
	password_hash::{PasswordHash, PasswordVerifier, rand_core::OsRng, PasswordHasher, SaltString},
};
use jwt_simple::prelude::*;
use tera::Context;

use crate::{
	common::{Templates},
	auth::views::{read_msg, update_details},
	auth::models::{
		FormUpdateUser, FormLogin, FormNewUser, VeriUser, AuToken
	}
};

pub async fn get_login(
	cookie: HeaderMap,
	Extension(templates): Extension<Templates>,
) -> impl IntoResponse {

	let msg = read_msg(cookie).await.unwrap();

	let mut context = Context::new();
	context.insert("msg", &msg.unwrap());
	Html(templates.render("login", &context).unwrap())
}

pub async fn post_login(
	State(pool): State<PgPool>,
	Extension(templates): Extension<Templates>,
	Form(form): Form<FormLogin>,
) -> Result<impl IntoResponse, impl IntoResponse> {

	let mut context = Context::new();
	let email = query!(
		"SELECT email FROM users WHERE email=$1",
		&form.email
	)
	.fetch_optional(&pool)
	.await
	.unwrap();
	if email.is_none() {
		context.insert("for_email", "this email is not available..!");
		return Err(Html(templates.render("login", &context).unwrap()));
	}

	let pass = query!(
		"SELECT password FROM users WHERE email=$1",
		&form.email
	)
	.fetch_one(&pool)
	.await
	.unwrap();

	let parsed_hash = PasswordHash::new(pass.password.as_str()).unwrap();
	let veri = Pbkdf2
		.verify_password(form.password.as_bytes(), &parsed_hash)
		.is_ok();
	if !veri {
		context.insert("for_password", "password is not correct..!");
		return Err(Html(templates.render("login", &context).unwrap()));
	};

	let in_user = query_as!(
		VeriUser,
		"SELECT id, email, username, status FROM users WHERE email=$1",
		&form.email
	)
	.fetch_optional(&pool)
	.await
	.unwrap();

	let user = match in_user {
		Some(user) => user,
		None => return Err(
			Html(templates.render("login", &context).unwrap())
		),
	};
	// ..token
	let obj = AuToken {
		id: user.id,
		email: user.email.clone(),
		username: user.username,
		status: user.status,
	};

	let de_key = RsaOaepDecryptionKey::generate(2048).unwrap();
	let en_key = de_key.encryption_key();

    if fs::exists(
    	"./static/de_key/user/".to_string() + &form.email
    ).unwrap() {
    	fs::remove_dir_all("./static/de_key/user/".to_string() + &form.email).unwrap();
    }
    let _ = fs::create_dir_all("./static/de_key/user/".to_string() + &form.email);

    let dialogue = Alphanumeric.sample_string(
        &mut rand::rng(), 12
    );
    let mut s = File::create(
    	format!("./static/de_key/user/{}/{}{}", form.email, dialogue, ".der")
    ).unwrap();

    //let dk: &[u8] = ;
    let _ = s.write_all(&de_key.to_der().unwrap());

	let claims = Claims::with_custom_claims(
		obj, Duration::from_hours(2)
	);
	let token = en_key.encrypt(claims).unwrap();

	Ok(Response::builder()
		.status(StatusCode::FOUND)
		.header("Location", "/account/users")
		.header(
			"Set-Cookie",
			format!(
				"{}={}; Path={}; HttpOnly={}; SameSite={}",
				"visit", token, "/", "true", "lax",
			),
		)
		.header(
			"Set-Cookie",
			format!(
				"{}={}; Path={}; HttpOnly={}; SameSite={}",
				"dialogue", dialogue, "/", "true", "lax",
			),
		)
		.header(
			"Set-Cookie",
			format!(
				"{}={}; Path={}; HttpOnly={}; SameSite={}",
				"user", form.email, "/", "true", "lax",
			),
		)
		.body(Body::from("not found")).unwrap()
	)
}


pub async fn get_signup(
    Extension(templates): Extension<Templates>
) -> impl IntoResponse {

    Html(templates.render("signup", &Context::new()).unwrap())
}
pub async fn post_signup(
    State(pool): State<PgPool>,
    Extension(templates): Extension<Templates>,
    Form(form): Form<FormNewUser>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();
    let q_email = sqlx::query!(
        "SELECT email FROM users WHERE email=$1",
        &form.email
    )
    .fetch_optional(&pool)
    .await;
    match q_email {
        Ok(None) => (),
        _ => {
            context.insert("err", "email already exists..");
            return Err(Html(templates.render("signup", &context).unwrap()));
        }
    };
    let q_name = sqlx::query!(
        "SELECT username FROM users WHERE username=$1",
        &form.username
    )
    .fetch_optional(&pool)
    .await;
    match q_name {
        Ok(None) => (),
        _ => {
            context.insert("err", "username already exists..");
            return Err(Html(templates.render("signup", &context).unwrap()));
        }
    };
    let salt = SaltString::generate(&mut OsRng);
    let pass = Pbkdf2.hash_password(form.password.as_bytes(), &salt);
    let hashed_password = match pass {
        Ok(expr) => expr.to_string(),
        Err(err) => return Err(Html("Error pass..!".to_owned() + &err.to_string())),
    };
    let status: Vec<String> = vec![];

    let _ = sqlx::query(
        "INSERT INTO users (email, username, password, status, created_at) VALUES ($1,$2,$3,$4,$5)",
    )
    .bind(&form.email)
    .bind(&form.username)
    .bind(&hashed_password)
    .bind::<Vec<_>>(status)
    .bind(Utc::now())
    .execute(&pool)
    .await
    .unwrap();
    Ok(Redirect::to("/account/users").into_response())
}

#[axum::debug_handler]
pub async fn get_update(
	i: Option<AuToken>,
    State(pool): State<PgPool>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

	let mut context = Context::new();
	let t = match i {
		Some(expr) => expr,
        None => {
        	context.insert("is_no", "Caramba bullfighting and damn it");
        	return Err(Html(templates.render("update", &context).unwrap()))
    	}
    };
    let user = update_details(pool, t.id).await;
    match user {
        Ok(user) => {
            context.insert("user", &user);
            Ok(Html(templates.render("update", &context).unwrap()))
        }
        Err(Some(err)) => {
        	context.insert("err", &err.to_string());
        	Err(Html(templates.render("update", &context).unwrap()))
        }
        Err(None) => {
        	context.insert("is_no", "Caramba bullfighting and damn it");
        	Err(Html(templates.render("update", &context).unwrap()))
        }
    }
}

pub async fn post_update_user(
	i: Option<AuToken>,
    State(pool): State<PgPool>,
    Form(form): Form<FormUpdateUser>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let t = match i {
        Some(expr) => expr,
        None => return Err(Redirect::to("/account/login").into_response()),
    };
    let _ = sqlx::query!(
        "UPDATE users SET email=$2, username=$3, updated_at=$4 WHERE id=$1",
        t.id,
        &form.email,
        &form.username,
        Some(Utc::now())
    )
    .fetch_one(&pool)
    .await;
    Ok(Response::builder()
        .status(StatusCode::FOUND)
        .header("Location", "/account/login")
        .header(
            "Set-Cookie",
            format!(
                "{}={}; Path={}; HttpOnly={}; SameSite={}",
                "visit", "_", "/", "true", "lax",
            ),
        )
        .body(Body::from("not found"))
        .unwrap())
}