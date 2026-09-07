use axum::{
    routing::get,
    Router,
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{
    frontend::assets,
    http::router,
};

pub fn create_app(state: crate::http::state::AppState) -> Router {
    Router::new()
        .merge(router::api_router(state))
        .route("/*path", get(assets::serve))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}