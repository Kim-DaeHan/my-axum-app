use axum::Router;

use crate::post::route::post_routes;

pub fn api_routes() -> Router {
    Router::new().nest("/post", post_routes())
}
