use sqlx::postgres::PgPool;
use std::sync::{Arc};
use axum::{
    routing::get,
    Extension, Router
};
use tera::Tera;

use crate::{
    auth::handlers,
};

pub fn build_rt(pool: PgPool) -> Router {
    let mut user_tera = Tera::default();
    user_tera
        .add_raw_templates(vec![
            ("base.html", include_str!("../../templates/base.html")),
            ("navbar.html", include_str!("../../templates/navbar.html")),
            ("rq_user.html", include_str!("../../templates/rq_user.html")),
            ("login", include_str!("../../templates/login.html")),
            ("users", include_str!("../../templates/users.html")),
        ])
        .unwrap();

    let auth_routes = Router::new().nest(
        "/account",
        Router::new()
            .route(
                "/users",
                get(handlers::users)
            )
            .route(
                "/login",
                get(handlers::get_login).post(handlers::post_login)
            )
            // .route(
            //     "/nullify",
            //     get(handlers::nullify)
            // )
            .layer(Extension(Arc::new(user_tera.clone())))
    );
    Router::new().merge(auth_routes.with_state(pool))

}
