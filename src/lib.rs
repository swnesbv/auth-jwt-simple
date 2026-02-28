pub mod common;
pub mod pgnation;

pub mod distribution {
    pub mod routes_index;
    pub mod routes_account;
    pub mod routes_assets;
}
pub mod auth {
    pub mod handlers;
    pub mod models;
    pub mod views;
}
pub mod util {
    pub mod date_config;
    pub mod r_body;
}