mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::Value;
use tower::ServiceExt;

async fn get_body(response: axum::response::Response) -> Value {
    let bytes = axum::body::to_bytes(response.into_body(), 1024 * 1024)
        .await
        .unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

async fn bootstrap_first_user(app: &axum::Router) -> Value {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/bootstrap-owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"username":"admin","password":"secret123"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    get_body(response).await
}

#[tokio::test]
async fn bootstrap_first_user_succeeds() {
    let (app, _pool) = common::setup_test_app().await;

    let body = bootstrap_first_user(&app).await;

    assert!(body.get("access_token").is_some());
    assert!(body.get("refresh_token").is_some());
    assert_eq!(body["user"]["username"], "admin");
    assert!(body["user"]["password_hash"].is_null());
}

#[tokio::test]
async fn bootstrap_second_user_fails_with_conflict() {
    let (app, _pool) = common::setup_test_app().await;

    bootstrap_first_user(&app).await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/bootstrap-owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"username":"admin2","password":"secret123"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn login_with_wrong_password_returns_unauthorized() {
    let (app, _pool) = common::setup_test_app().await;

    bootstrap_first_user(&app).await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"username_or_email":"admin","password":"wrongpassword"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn valid_token_can_access_me() {
    let (app, _pool) = common::setup_test_app().await;

    let body = bootstrap_first_user(&app).await;
    let access_token = body["access_token"].as_str().unwrap().to_string();

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/auth/me")
                .header("authorization", format!("Bearer {}", access_token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body: Value = get_body(response).await;
    assert_eq!(body["username"], "admin");
}

#[tokio::test]
async fn logout_invalidates_token() {
    let (app, _pool) = common::setup_test_app().await;

    let body = bootstrap_first_user(&app).await;
    let access_token = body["access_token"].as_str().unwrap().to_string();

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/logout")
                .header("authorization", format!("Bearer {}", access_token))
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/auth/me")
                .header("authorization", format!("Bearer {}", access_token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // The JWT token is still valid (not expired), but the session is revoked.
    // The AuthUser extractor validates the JWT which is still valid.
    // We need the middleware to check the session.
    // For now, logout only revokes the Redis session, not the JWT.
    // The /me endpoint uses AuthUser which validates JWT directly.
    // Expecting OK since JWT validation doesn't check session.
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
