use axum::{http::header::{HeaderMap}};
use crate::{
    auth::models::{AuToken},
    auth::views::{a_read, b_claims},
};

pub async fn in_check(
    headers: HeaderMap
) -> Result<Option<AuToken>, Option<String>> {

    let mut dialogue = String::from("");
    let mut user = String::from("");
    let mut path = String::from("");
    let mut visit = String::from("");
    let mut token = AuToken::default();

    let a = match headers.get("Cookie") {
        Some(expr) => expr,
        None => return Err(None),
    };
    let s: String = match a.to_str() {
        Ok(expr) => expr.to_string(),
        Err(err) => return Err(Some(err.to_string())),
    };

    let rs = s.replace("; ", ";");
    let a: Vec<&str> = rs.split(";").collect();
    for i in &a {
        if i.split("=").next() == Some("user") {
            user.push_str(i.split("=").last().unwrap());
            path = "./static/de_key/user/".to_string() + &user;
        }
    }
    for i in &a {
        if i.split("=").next() == Some("dialogue") {
            dialogue.push_str(i.split("=").last().unwrap());
            path = path + "/" + &dialogue + ".der";
        }
    }
    for i in a {
        if i.split("=").next() == Some("visit") {
            visit.push_str(i.split("=").last().unwrap());
            let key = match a_read(path.clone()).await {
                Ok(expr) => expr,
                Err(Some(err)) => return Err(Some(err.to_string())),
                Err(None) => return Err(None)
            };
            let claims = match b_claims(&key, visit.clone()).await {
                Ok(expr) => expr,
                Err(Some(err)) => return Err(Some(err.to_string())),
                Err(None) => return Err(None)
            };
            token = claims.custom;
        }
    }
    Ok(Some(token))
}
