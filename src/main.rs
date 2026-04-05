use std::sync::Arc;
use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;
use axum::Router;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

use demo::auth::models::{AuthRedis};
use demo::distribution::routes_index;
use demo::distribution::routes_account;
use demo::distribution::routes_assets;

#[tokio::main]
async fn main() {

    //..Postgres
    let cfg = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = PostgresConnectionManager::new_from_stringlike(cfg, NoTls).unwrap();
    let pool = bb8::Pool::builder().build(manager).await.unwrap();
    //..Redis
    let client = redis::Client::open("redis://localhost").unwrap();
    let conn = bb8::Pool::builder().build(client).await.unwrap();
    // ..
    let a = AuthRedis {
        pool: pool.clone(),
        conn: conn.clone()
    };
    let index_router = routes_index::rt(Arc::new(a.clone())).await;
    let account_router = routes_account::rt(Arc::new(a.clone())).await;
    let assets_router = routes_assets::rt();

    let app = Router::new()
        .merge(index_router)
        .merge(account_router)
        .merge(assets_router);

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8000));
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!(" listening on.. {:?}", addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}