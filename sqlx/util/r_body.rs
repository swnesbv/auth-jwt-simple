use serde::Deserialize;

use core::marker::PhantomData;
use serde::de::{Deserializer, MapAccess, Visitor};

use axum::{
    body::Bytes,
    extract::{FromRequest, Request},
    response::{IntoResponse, Response},
};

pub struct InputBody(pub Bytes);
impl<S> FromRequest<S> for InputBody
where
    Bytes: FromRequest<S>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let body = Bytes::from_request(req, state)
            .await
            .map_err(IntoResponse::into_response)?;
        Ok(Self(body))
    }
}

struct ListInput<V>(PhantomData<fn() -> V>);
impl<'de, V: Deserialize<'de>> Visitor<'de> for ListInput<V> {
    type Value = Option<Vec<V>>;

    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("err..")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut s: Option<Vec<V>> = Some(Vec::with_capacity(map.size_hint().unwrap_or(0)));

        while let Some((key, value)) = map.next_entry::<String, V>()? {
            if key == "list" {
                s.as_mut().expect("REASON").push(value);
            }
        }
        Ok(Some(s.expect("REASON")))
    }
}
pub fn deserialize_list<'de, D, V>(deserializer: D) -> Result<Option<Vec<V>>, D::Error>
where
    D: Deserializer<'de>,
    V: Deserialize<'de>,
{
    deserializer.deserialize_map(ListInput(PhantomData))
}

pub fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;

    match s {
        "on" => Ok(true),
        "off" => Ok(false),
        _ => Err(serde::de::Error::unknown_variant(s, &["on", "off"])),
    }
}

pub fn to_bool(v: Vec<&str>) -> Vec<bool> {
    let mut x: Vec<bool> = vec![];
    for i in v {
        match i {
            "true" => x.push(true),
            "t" => x.push(true),
            "false" => x.push(false),
            "f" => x.push(false),
            "yes" => x.push(true),
            "y" => x.push(true),
            "no" => x.push(false),
            "n" => x.push(false),
            "1" => x.push(true),
            "0" => x.push(false),
            _ => panic!("err.. {:?}", i),
        };
    }
    x
}

/*fn main() {
    let a = vec!["1", "0", "1", "0"];
    let b = vec![1, 2, 3, 4];
    let c = to_bool(a);
    println!("c = {:?}", c);

    let mut f: Vec<i32> = vec![];
    for (d,e) in c.iter().zip(b.iter()) {
        if *d == true {
            f.push(*e);
        }
    }
    println!("f.. {:?}", f);
}*/
