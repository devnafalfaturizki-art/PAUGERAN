use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use paugeran::{
    database::Database,
    http::{app::create_app, state::AppState},
};
use tower::ServiceExt;

async fn test_app() -> axum::Router {
    let directory = tempfile::tempdir().expect("temporary database directory");
    let database = Database::connect(directory.path().to_str().unwrap())
        .await
        .unwrap();
    database.seed_if_empty().await.unwrap();
    create_app(AppState {
        started_at: chrono::Utc::now(),
        database,
        pii_redactor: paugeran::crypto::pii_redactor::PiiRedactor::default(),
    })
}

#[tokio::test]
async fn health_endpoint_returns_ok() {
    let response = test_app()
        .await
        .oneshot(Request::get("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn cases_endpoint_returns_seeded_case() {
    let response = test_app()
        .await
        .oneshot(Request::get("/api/cases").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let cases: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(cases.as_array().unwrap().len(), 1);
}
