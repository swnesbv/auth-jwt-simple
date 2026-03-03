
use axum::{http::header::{HeaderMap}};
use crate::{
    auth::models::{AuToken},
    auth::views::{a_read, b_claims},
};


pub async fn in_check(
    headers: HeaderMap
) -> Result<Option<AuToken>, Option<String>> {
    match headers.get("Cookie") {
        None => return Ok(None),
        Some(expr) => expr,
    };
    let mut dialogue = String::from("");
    let mut user = String::from("");
    let mut path = String::from("");
    let mut visit = String::from("");
    let mut token = AuToken::default();

    let s = headers.get("Cookie").unwrap().to_str().unwrap();
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
                Err(err) => return Err(Some(err.expect("REASON").to_string()))
            };
            let claims = match b_claims(&key, visit.clone()).await {
                Ok(expr) => expr,
                Err(err) => return Err(Some(err.expect("REASON").to_string()))
            };
            token = claims.custom;
        }
    }
    Ok(Some(token))
}


/*pub async fn in_check(
    headers: HeaderMap
) -> Result<Option<AuToken>, Option<String>> {
    match headers.get("Cookie") {
        None => return Ok(None),
        Some(expr) => expr,
    };
    let mut dialogue = String::from("");
    let mut user = String::from("");
    let mut path = String::from("");
    let mut visit = String::from("");
    let mut token = AuToken::default();
    let mut claims: JWTClaims<AuToken>;

    let s = headers.get("Cookie").unwrap().to_str().unwrap();
    let rs = s.replace("; ", ";");
    let a: Vec<&str> = rs.split(";").collect();
    for i in &a {
        if i.split("=").next() == Some("user") {
            user.push_str(i.split("=").last().unwrap());
            path = "./static/public_key/user/".to_string() + &user;
        }
    }
    println!(" 1 path..! {:?}", path);
    for i in &a {
        if i.split("=").next() == Some("dialogue") {
            dialogue.push_str(i.split("=").last().unwrap());
            path = path + "/" + &dialogue + ".bin";
        }
    }
    println!(" 2 path..! {:?}", path);
    for i in a {
        if i.split("=").next() == Some("visit") {
            visit.push_str(i.split("=").last().unwrap());
            let key: &[u8] = &fs::read(&path).unwrap();
            let k = ES256PublicKey::from_bytes(key);
            claims = k.unwrap().verify_token::<AuToken>(&visit, None).unwrap();
            token = claims.custom;
        }
    }
    Ok(Some(token))
}*/
