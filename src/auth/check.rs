use axum::{http::header::{HeaderMap}};
use redis::AsyncCommands;

use crate::{
    common::{RedisPool},
    auth::models::{AuToken, KeyEmail},
    auth::views::{a_read, b_claims},
};

pub async fn in_check(
    conn: RedisPool, headers: HeaderMap,
) -> Result<Option<AuToken>, Option<String>> {

    let mut visit = String::from("");
    let mut token = AuToken::default();
    let mut session = String::from("");

    let a = match headers.get("Cookie") {
        Some(expr) => expr,
        None => return Err(None),
    };
    let s: String = match a.to_str() {
        Ok(expr) => expr.to_string(),
        Err(err) => return Err(Some(err.to_string())),
    };
    let ss = s.replace("; ", ";");
    let all: Vec<&str> = ss.split(";").collect();
    for i in &all {
        if i.split("=").next() == Some("sess") {
            session.push_str(i.split("=").last().unwrap());
        }
    }
    // ..Redis
    let mut rs = match conn.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let sess:  String = rs.get(session).await.unwrap_or("No..".to_string());
    let obj:   KeyEmail = serde_json::from_str(&sess).unwrap();
    let key:   String = obj.key;
    let email: String = obj.email;
    let path = "./static/de_key/user/".to_string() + &email + "/" + &key + ".der";
    //..
    for i in all {
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
