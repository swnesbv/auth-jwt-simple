
use std::sync::{Arc};
use axum::{
    routing::{get},
    Extension, Router
};
use tera::Tera;

use crate::{
    auth::handlers,
    auth::accreditation,
    auth::models::{AuthRedis},
};

pub async fn rt(state: Arc<AuthRedis>) -> Router {
    let mut user_tera = Tera::default();
    user_tera
        .add_raw_templates(vec![
            ("base.html", include_str!("../../templates/base.html")),
            ("navbar.html", include_str!("../../templates/navbar.html")),
            ("rq_user.html", include_str!("../../templates/rq_user.html")),
            ("login", include_str!("../../templates/login.html")),
            ("signup", include_str!("../../templates/signup.html")),
            ("update", include_str!("../../templates/update.html")),
            ("users", include_str!("../../templates/users.html")),
            (
                "password_change",
                include_str!("../../templates/password_change.html"),
            ),
        ])
        .unwrap();

    let auth_routes = Router::new().without_v07_checks().nest(
        "/account",
        Router::new()
            .route(
                "/login",
                get(accreditation::get_login)
                .post(accreditation::post_login)
            )
            .route(
                "/signup",
                get(accreditation::get_signup)
                .post(accreditation::post_signup),
            )
            .route(
                "/update",
                get(accreditation::get_update)
                .post(accreditation::post_update_user),
            )
            .route(
                "/users",
                get(handlers::users)
            )
            .without_v07_checks()
            .route("/user/{name}", get(handlers::user))
            .route(
                "/password-change",
                get(accreditation::get_password_change)
                    .post(accreditation::post_password_change),
            )
            // .route(
            //     "/nullify",
            //     get(accreditation::nullify)
            // )
            .layer(Extension(Arc::new(user_tera.clone())))
    );
    Router::new().merge(auth_routes.with_state(state))
}
