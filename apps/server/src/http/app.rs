use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{
    frontend::assets,
    http::{router, state::AppState},
};

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .merge(router::api_router(state.clone()))
        .route("/*path", get(assets::serve))
        .with_state(Arc::new(state))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}
