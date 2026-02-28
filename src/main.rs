use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;
use sqlx::PgPool;
use axum::Router;

use demo::distribution::routes_index;
use demo::distribution::routes_account;
use demo::distribution::routes_assets;

#[tokio::main]
async fn main() {

    let cfg = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&cfg).await.unwrap();

    let index_router = routes_index::build_rt(pool.clone()).await;
    let account_router = routes_account::build_rt(pool.clone());
    let assets_router = routes_assets::build_rt();

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