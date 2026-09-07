use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use paugeran::{
    database::Database,
    http::{app::create_app, state::AppState},
};
use tower::ServiceExt;

#[tokio::test]
async fn message_endpoint_returns_structured_uncertainty() {
    let directory = tempfile::tempdir().unwrap();
    let database = Database::connect(directory.path().to_str().unwrap())
        .await
        .unwrap();
    database.seed_if_empty().await.unwrap();
    let case_id = database.list_cases().await.unwrap().remove(0).id;
    let app = create_app(AppState {
        started_at: chrono::Utc::now(),
        database,
        pii_redactor: paugeran::crypto::pii_redactor::PiiRedactor::default(),
    });
    let request = Request::post(format!("/api/cases/{case_id}/messages"))
        .header("content-type", "application/json")
        .body(Body::from(r#"{"content":"Fakta awal perkara"}"#))
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let result: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(result["certaintyScore"].as_f64().unwrap() >= 0.0);
    assert!(result["clarifyingQuestions"].as_array().unwrap().len() >= 2);
}

#[tokio::test]
async fn stream_endpoint_returns_sse_content_type() {
    let directory = tempfile::tempdir().unwrap();
    let database = Database::connect(directory.path().to_str().unwrap())
        .await
        .unwrap();
    database.seed_if_empty().await.unwrap();
    let case_id = database.list_cases().await.unwrap().remove(0).id;
    let app = create_app(AppState {
        started_at: chrono::Utc::now(),
        database,
        pii_redactor: paugeran::crypto::pii_redactor::PiiRedactor::default(),
    });
    let response = app
        .oneshot(
            Request::get(format!("/api/cases/{case_id}/messages/stream"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()["content-type"], "text/event-stream");
}
