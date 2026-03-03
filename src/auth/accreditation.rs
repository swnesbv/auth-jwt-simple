use std::fs;
use std::fs::File;
use std::io::prelude::*;
use sqlx::postgres::PgPool;
use sqlx::{query, query_as};
use axum::{
	body::Body,
	extract::{Form, State},
	http::{Response, StatusCode, header::{HeaderMap}},
	response::{Html, IntoResponse},
	Extension,
};
use rand::distr::{Alphanumeric, SampleString};
use pbkdf2::{
	password_hash::{PasswordHash, PasswordVerifier}, Pbkdf2,
};
use jwt_simple::prelude::*;
use tera::Context;

use crate::{
	common::{Templates},
	auth::views::{read_msg},
	auth::models::{
		FormLogin, VeriUser, AuToken},
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


/*pub async fn post_login(
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

    let key_pair = ES256KeyPair::generate();
    let public_key = key_pair.public_key();

    if fs::exists("./static/public_key/user/".to_string() + &form.email).unwrap() {
    	fs::remove_dir_all("./static/public_key/user/".to_string() + &form.email).unwrap();
    }
    let _ = fs::create_dir_all("./static/public_key/user/".to_string() + &form.email);

    let dialogue = Alphanumeric.sample_string(
        &mut rand::rng(), 12
    );
    let mut s = File::create(
    	format!("./static/public_key/user/{}/{}{}", form.email, dialogue, ".bin")
    ).unwrap();
    s.write_all(&public_key.to_bytes()).unwrap();

	let claims = Claims::with_custom_claims(
		obj, Duration::from_hours(2)
	);
	let token = key_pair.sign(claims).unwrap();

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
}*/