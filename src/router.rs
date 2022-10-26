use axum::{routing::get, Router};

use crate::module::user::router;

async fn root() -> &'static str {
    "root api"
}

pub fn api_router() -> Router {
    // build our application with a route
    let root_route = Router::new().route("/", get(root));

    // This is the order that the modules were authored in.
    root_route.merge(router::router())
}
