use std::fs;
use jwt_simple::prelude::*;
use axum::{http::header::{HeaderMap, HeaderValue}};
use chrono::{DateTime, NaiveDateTime, Utc};

use crate::{
    common::{PgPool},
    auth::models::{ListUser, UpdateUser, AuToken},
};

pub async fn some_headers(
    headers: HeaderMap
) -> Option<HeaderValue> {

    let a = match headers.get("Cookie") {
        Some(expr) => expr,
        None => return None
    };
    Some(a.clone())
}
pub async fn a_check(
    headers: HeaderMap
) -> Result<Option<String>, Option<String>> {

    let a = match some_headers(headers).await {
        Some(expr) => expr,
        None => return Ok(None)
    };
    let b = match a.to_str() {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string())),
    };
    Ok(Some(b.to_string()))
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
        Some(expr) => expr,
        None => return Ok(None),
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


pub async fn all(
    pool: PgPool,
) -> Result<Vec<ListUser>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };

    let result = pg.query("SELECT id, email, username, img, created_at, updated_at FROM users;", &[])
    .await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };

    let r: Vec<ListUser> = rows.into_iter().map(|i| ListUser {id: i.get(0), email: i.get(1), username: i.get(2), img: i.get(3), created_at: i.get(4), updated_at: i.get(5)})
        .collect::<Vec<ListUser>>();
    Ok(r)
}

pub async fn update_details(
    pool: PgPool, id: i32
) -> Result<UpdateUser, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query("SELECT email, username, updated_at FROM users WHERE id=$1", &[&id])
    .await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };

    let into_utc = |timestamp: NaiveDateTime| DateTime::<Utc>::from_naive_utc_and_offset(timestamp, Utc);
    let i = &rows[0];
    let timestamp: NaiveDateTime = i.get("updated_at");
    let t = into_utc(timestamp);
    let r = UpdateUser {email: i.get(0), username: i.get(1), updated_at: Some(t)};
    Ok(r)
}

pub async fn details(
    pool: PgPool,
    name: String
) -> Result<ListUser, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query(
        "SELECT id, email, username, img, created_at, updated_at FROM users WHERE username=$1",
        &[&name]
    )
    .await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let i = &rows[0];
    let r = ListUser {
        id:         i.get(0),
        email:      i.get(1),
        username:   i.get(2),
        img:        i.get(3),
        created_at: i.get(4),
        updated_at: i.get(5)
    };
    Ok(r)
}


pub async fn headers_in(
    headers: HeaderMap
) -> Result<HeaderValue, Option<String>> {
    match headers.get("Cookie") {
        None => Err(None),
        Some(expr) => Ok(expr.clone()),
    }
}
