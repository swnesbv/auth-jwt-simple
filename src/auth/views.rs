use std::fs;
use jwt_simple::prelude::*;
use axum::{http::header::{HeaderMap, HeaderValue}};
use sqlx::postgres::PgPool;

use crate::{
    auth::models::{UpdateUser, ListUser, AuToken},
};

pub async fn update_details(
    pool: PgPool, id: i32
) -> Result<UpdateUser, Option<sqlx::Error>> {

    let result = sqlx::query_as!(
        UpdateUser,
        "SELECT email, username, updated_at FROM users WHERE id=$1",
        id
    )
    .fetch_one(&pool)
    .await;
    let r = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err))
    };
    Ok(r)
}

pub async fn a_read(
    path: String,
) -> Result<Vec<u8>, Option<std::io::Error>> {
    let key = match fs::read(path) {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err))
    };
    Ok(key.to_vec())
}
pub async fn b_claims(
    key: &[u8], visit: String,
) -> Result<JWTClaims<AuToken>, Option<jwt_simple::Error>> {
    let k = match RsaOaepDecryptionKey::from_der(key) {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err))
    };
    let claims = match k.decrypt_token::<AuToken>(&visit, None) {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err))
    };
    Ok(claims)
}

pub async fn cookie_check(
    headers: HeaderMap
) -> bool {
    if headers.get("Cookie").is_some() {
        return true;
    }
    false
}
pub async fn get_cookie(
    name: String,
    headers: HeaderMap
) -> String {
    if cookie_check(headers.clone()).await {
        let s = headers.get("Cookie").unwrap().to_str().unwrap();
        let rs = s.replace("; ", ";");
        let a: Vec<&str> = rs.split(";").collect();
        let mut b = String::from("");
        for i in a {
            if i.split("=").next() == Some(&name) {
               b.push_str(i.split("=").last().unwrap())
            }
        }
        b
    } else {
        "None".to_string()
    }
}

pub async fn some_cookie(
    name: String,
    headers: HeaderMap
) -> Option<String> {
    if cookie_check(headers.clone()).await {
        let s = headers.get("Cookie").unwrap().to_str().unwrap();
        let rs = s.replace("; ", ";");
        let a: Vec<&str> = rs.split(";").collect();
        let mut b = String::from("");
        for i in a {
            if i.split("=").next() == Some(&name) {
               b.push_str(i.split("=").last().unwrap())
            }
        }
        Some(b)
    } else {
        Some("".to_string())
    }
}

pub async fn parse_cookie(
    name: String,
    headers: HeaderMap
) -> Result<Option<String>, Option<String>> {

    let _ = match headers.get("Cookie") {
        None => return Ok(None),
        Some(expr) => expr,
    };
    let s = headers["Cookie"].to_str().unwrap();
    let rs = s.replace("; ", ";");
    let a: Vec<&str> = rs.split(";").collect();
    let mut c = String::from("");
    for i in a {
        if i.split("=").next() == Some(&name) {
           c.push_str(i.split("=").last().unwrap())
        }
    }
    Ok(Some(c))
}

pub async fn read_msg(
    cookie: HeaderMap
) -> Result<Option<Vec<String>>, Option<String>> {

    let token = some_cookie(
        "to_msg".to_string(), cookie
    ).await.unwrap_or("".to_string());

    let v: Vec<&str> = token.split(",").collect();
    let mut vec = Vec::new();
    for i in v {
        vec.push(i.to_string());
    }
    Ok(Some(vec))
}

pub async fn all(pool: PgPool) -> Result<Vec<ListUser>, String> {
    let result = sqlx::query_as!(
        ListUser,
        "SELECT id, email, username, img, created_at, updated_at FROM users"
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    Ok(result)
}
pub async fn headers_in(
    headers: HeaderMap
) -> Result<HeaderValue, Option<String>> {
    match headers.get("Cookie") {
        None => Err(None),
        Some(expr) => Ok(expr.clone()),
    }
}


/*pub async fn ss_add(session: Session, email: &str) -> Json<Value> {
    let saved_int = match session.get::<String>("email").await {
        Ok(i) => i,
        Err(error) => {
            println!("Error getting key: {}", error);
            None
        }
    };
    println!("saved int: {:?}", saved_int);
    let int = match saved_int {
        Some(i) => i,
        None => {
            println!("Saved int not found.");
            let new = email;
            if let Err(error) = session.insert("email", new).await {
                println!("Error inserting: {}", error)
            };
            new.to_string()
        }
    };

    Json(json!({
        "data": int
    }))
}*/
/*let mut b = String::from("");
let n = "a";

let s = &headers["Cookie"].to_str().unwrap();
let rs = s.replace("; ", ";");
let a: Vec<&str> = rs.split(";").collect();

for i in a {
    if i.split("=").next() == Some(n) {
       b.push_str(i.split("=").last().expect("REASON"))
    }
}
println!("b.. {:?}", b);*/