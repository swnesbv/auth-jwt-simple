use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use redis::AsyncCommands;
use axum::{
	body::Body,
	extract::{Form, State},
	http::{Response, StatusCode, header::{HeaderMap}},
	response::{Html, IntoResponse, Redirect},
	Extension,
};
use chrono::{Utc};
use rand::distr::{Alphanumeric, SampleString};
use pbkdf2::{Pbkdf2,
	password_hash::{PasswordHash, PasswordVerifier, rand_core::OsRng, PasswordHasher, SaltString},
};
use jwt_simple::prelude::*;
use tera::Context;

use crate::{
	common::{Templates, DoubleConn},
	auth::views::{read_msg, update_details},
	auth::models::{
		FormUpdateUser, FormLogin, FormNewUser, AuToken
		//VeriUser
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
	State(dc): State<Arc<DoubleConn>>,
	Extension(templates): Extension<Templates>,
	Form(form): Form<FormLogin>,
) -> Result<impl IntoResponse, impl IntoResponse> {

	let mut context = Context::new();

    let pg = match dc.pool.get().await{
        Ok(expr) => expr,
        Err(err) => {
        	context.insert("err", &err.to_string());
        	return Err(Html(templates.render("login", &context).unwrap()))
        }
    };

	let email = pg.query_one(
		"SELECT email FROM users WHERE email=$1;",
		&[&form.email]
	)
	.await
	.unwrap();
	if email.is_empty() {
		context.insert("for_email", "this email is not available..!");
		return Err(Html(templates.render("login", &context).unwrap()));
	}
	let pass = pg.query_one(
		"SELECT password FROM users WHERE email=$1;", &[&form.email]
	)
	.await
	.unwrap();
	let rpass: &str = pass.get(0);
	let parsed_hash = PasswordHash::new(rpass).unwrap();
	let veri = Pbkdf2
		.verify_password(form.password.as_bytes(), &parsed_hash)
		.is_ok();
	if !veri {
		context.insert("for_password", "password is not correct..!");
		return Err(Html(templates.render("login", &context).unwrap()));
	};

	let in_user = pg.query(
		"SELECT id, email, username, status FROM users WHERE email=$1", &[&form.email]
	)
	.await;
	let user = match in_user {
		Ok(expr) => expr,
		Err(err) => {
			context.insert("err", &err.to_string());
			return Err(Html(templates.render("login", &context).unwrap()))
		}
	};
	// ..Token
	let row = &user[0];
	let obj = AuToken {
		id: row.get(0),
		email: row.get(1),
		username: row.get(2),
		status: row.get(3),
	};
	// ..
	let de_key = RsaOaepDecryptionKey::generate(2048).unwrap();
	let en_key = de_key.encryption_key();
    let dialogue = Alphanumeric.sample_string(
        &mut rand::rng(), 12
    );

    if fs::exists(
    	"./static/de_key/user/".to_string() + &form.email
    ).unwrap() {
    	fs::remove_dir_all("./static/de_key/user/".to_string() + &form.email).unwrap();
    }
    let _ = fs::create_dir_all(
    	"./static/de_key/user/".to_string() + &form.email
    );
    let mut s = File::create(
    	format!("./static/de_key/user/{}/{}{}", form.email, dialogue, ".der")
    ).unwrap();

    //Redis..
    let mut rs = match dc.conn.get().await{
        Ok(expr) => expr,
        Err(err) => {
        	context.insert("err", &err.to_string());
        	return Err(Html(templates.render("index", &context).unwrap()))
        }
    };
	rs.set::<&str, &str, ()>("email", &form.email).await.unwrap();
	rs.set::<&str, &str, ()>("key", &dialogue).await.unwrap();
    let r: String = rs.get("key").await.unwrap();
    println!(" result..! {:?}", r);

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
		.body(Body::from("not found")).unwrap()
	)
}


pub async fn get_signup(
    Extension(templates): Extension<Templates>
) -> impl IntoResponse {

    Html(templates.render("signup", &Context::new()).unwrap())
}
pub async fn post_signup(
    State(dc): State<Arc<DoubleConn>>,
    Extension(templates): Extension<Templates>,
    Form(form): Form<FormNewUser>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();

    let pg = match dc.pool.get().await{
        Ok(expr) => expr,
        Err(err) => {
        	context.insert("err", &err.to_string());
        	return Err(Html(templates.render("signup", &context).unwrap()))
        }
    };

    let q_email = pg.query_one(
		"SELECT email FROM users WHERE email=$1;",
		&[&form.email]
    )
    .await;
    let _ = match q_email {
		Ok(_) => {
			context.insert("err", "email already exists..");
			Ok(Html(templates.render("login", &context).unwrap()))
		}
		Err(err) => {
			context.insert("err", &err.to_string());
			Err(Html(templates.render("login", &context).unwrap()))
		}
    };
    let q_name = pg.query_one(
        "SELECT username FROM users WHERE username=$1",
        &[&form.username]
    )
    .await;
    let _ = match q_name {
		Ok(_) => {
			context.insert("err", "username already exists..");
			Ok(Html(templates.render("login", &context).unwrap()))
		}
		Err(err) => {
			context.insert("err", &err.to_string());
			Err(Html(templates.render("login", &context).unwrap()))
		}
    };
    let salt = SaltString::generate(&mut OsRng);
    let pass = Pbkdf2.hash_password(form.password.as_bytes(), &salt);
    let hashed_password = match pass {
        Ok(expr) => expr.to_string(),
        Err(err) => return Err(Html("Error pass..!".to_owned() + &err.to_string())),
    };
    let status: Vec<String> = vec![];
    let utc = Utc::now().naive_utc();
    let _ = pg.execute(
        "INSERT INTO users (email, username, password, status, created_at) VALUES ($1,$2,$3,$4,$5)",
        &[&form.email, &form.username, &hashed_password, &status, &utc]
    )
    .await
    .unwrap();
    Ok(Redirect::to("/account/users").into_response())
}

pub async fn get_update(
	i: Option<AuToken>,
    State(dc): State<Arc<DoubleConn>>,
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
    let user = update_details(dc.pool.clone(), t.id).await;
    match user {
        Ok(expr) => {
            context.insert("user", &expr);
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
    State(dc): State<Arc<DoubleConn>>,
    Extension(templates): Extension<Templates>,
    Form(form): Form<FormUpdateUser>,
) -> Result<impl IntoResponse, impl IntoResponse> {

	let mut context = Context::new();

    let t = match i {
        Some(expr) => expr,
        None => {
        	context.insert("err", "None AuToken..!");
        	return Err(Html(templates.render("update", &context).unwrap()))
        }
    };
    let pg = match dc.pool.get().await{
        Ok(expr) => expr,
        Err(err) => {
        	context.insert("err", &err.to_string());
        	return Err(Html(templates.render("update", &context).unwrap()))
        }
    };

    let utc = Utc::now().naive_utc();
    let _ = pg.execute(
        "UPDATE users SET email=$2, username=$3, updated_at=$4 WHERE id=$1",
        &[&t.id, &form.email, &form.username, &Some(utc)]
    )
    .await
    .unwrap();
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