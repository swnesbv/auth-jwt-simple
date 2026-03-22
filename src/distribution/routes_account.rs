
use std::sync::{Arc};
use axum::{
    routing::{get},
    Extension, Router
};
use tera::Tera;

use crate::{
    auth::handlers,
    auth::accreditation,
    common::{DoubleConn},
};

pub async fn rt(state: Arc<DoubleConn>) -> Router {
    let mut user_tera = Tera::default();
    user_tera
        .add_raw_templates(vec![
            ("base.html", include_str!("../../templates/base.html")),
            ("navbar.html", include_str!("../../templates/navbar.html")),
            ("rq_user.html", include_str!("../../templates/rq_user.html")),
            ("login", include_str!("../../templates/login.html")),
            ("signup", include_str!("../../templates/signup.html")),
            ("update", include_str!("../../templates/update.html")),
            ("i_users", include_str!("../../templates/i_users.html")),
            ("users", include_str!("../../templates/users.html")),
        ])
        .unwrap();

    let auth_routes = Router::new().nest(
        "/account",
        Router::new()
            .route(
                "/login",
                get(accreditation::get_login)
                .post(accreditation::post_login)
            )
            .route(
                "/signup",
                get(accreditation::get_signup).post(accreditation::post_signup),
            )
            .route(
                "/update",
                get(accreditation::get_update).post(accreditation::post_update_user),
            )
            .route(
                "/i_users",
                get(handlers::i_users)
            )
            .route(
                "/users",
                get(handlers::users)
            )
            // .route(
            //     "/nullify",
            //     get(accreditation::nullify)
            // )
            .layer(Extension(Arc::new(user_tera.clone())))
    );
    Router::new().merge(auth_routes.with_state(state))
}
