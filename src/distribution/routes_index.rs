use std::sync::{Arc};
use axum::{routing::get, Extension, Router};
use tera::Tera;

use crate::{
    auth::handlers,
    auth::models::{AuthRedis},
};

pub async fn rt(state: Arc<AuthRedis>) -> Router {
    let mut base_tera = Tera::default();
    base_tera
        .add_raw_templates(vec![
            ("base.html", include_str!("../../templates/base.html")),
            ("navbar.html", include_str!("../../templates/navbar.html")),
            ("rq_user.html", include_str!("../../templates/rq_user.html")),
            ("index", include_str!("../../templates/index.html")),
        ])
        .unwrap();

    let index_routes = Router::new()
        .route("/", get(handlers::index))
        .layer(Extension(Arc::new(base_tera.clone())));
    Router::new().merge(index_routes.with_state(state))
}
