use axum::{routing::get, Router};

use super::controller;

pub fn router() -> Router {
    Router::new().route("/api/v1/users", get(controller::users))
}
