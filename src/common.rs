use std::sync::Arc;
use tera::Tera;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::{request::Parts},
};
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;


pub type PgPool = bb8::Pool<PostgresConnectionManager<NoTls>>;

pub type RedisPool = bb8::Pool<redis::Client>;


#[derive(Debug)]
pub struct RedisConn(pub bb8::PooledConnection<'static, redis::Client>);
impl<S> FromRequestParts<S> for RedisConn
where
    RedisPool: FromRef<S>,
    S: Send + Sync + Clone + 'static,
{
    type Rejection = String;
    async fn from_request_parts(
    	_parts: &mut Parts, state: &S
    ) -> Result<Self, Self::Rejection> {
        let pool = RedisPool::from_ref(state);
        let conn = pool.get_owned().await.unwrap();
        Ok(Self(conn))
    }
}

#[derive(Clone, Debug)]
pub struct DoubleConn {
    pub pool: PgPool,
    pub conn: RedisPool
}


pub type Templates = Arc<Tera>;
