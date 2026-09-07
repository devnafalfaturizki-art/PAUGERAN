use axum::{
    extract::Path,
    http::{header, HeaderValue, StatusCode},
    response::Response,
};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../web/dist/"]
struct FrontendAssets;

pub async fn serve(Path(requested): Path<String>) -> Response {
    let asset_path = if requested.is_empty() {
        "index.html"
    } else {
        requested.as_str()
    };
    let asset = FrontendAssets::get(asset_path).or_else(|| FrontendAssets::get("index.html"));
    let Some(asset) = asset else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let content_type = mime_guess::from_path(asset_path)
        .first_or_octet_stream()
        .to_string();
    Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_TYPE,
            HeaderValue::from_str(&content_type).unwrap(),
        )
        .body(axum::body::Body::from(asset.data.into_owned()))
        .unwrap()
}

use axum::response::IntoResponse;
