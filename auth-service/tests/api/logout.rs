use auth_service::{utils::constants::JWT_COOKIE_NAME, ErrorResponse};
use reqwest::Url;

use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    let mut app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());

    let token = auth_cookie.value();
    let response = app.post_logout().await;

    let status = response.status().as_u16();
    assert_eq!(status, 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(auth_cookie.value().is_empty());

    let banned_token_store = app.banned_token_store.read().await;
    let is_banned = banned_token_store.is_token_banned(token).await.is_ok();

    assert!(is_banned);
    drop(banned_token_store);

    TestApp::cleanup(&mut app).await;
}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
    let mut app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());

    let response = app.post_logout().await;

    let status = response.status().as_u16();
    assert_eq!(status, 200);

    let response = app.post_logout().await;

    let status = response.status().as_u16();
    assert_eq!(status, 400);

    let body: ErrorResponse = response
        .json()
        .await
        .expect("Failed to parse response body");
    assert_eq!(body.error, "Missing auth token");

    TestApp::cleanup(&mut app).await;
}

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let mut app = TestApp::new().await;

    let response = app.post_logout().await;

    let status = response.status().as_u16();
    assert_eq!(status, 400);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME);

    assert!(auth_cookie.is_none());

    let body: ErrorResponse = response
        .json()
        .await
        .expect("Failed to parse response body");
    assert_eq!(body.error, "Missing auth token");

    TestApp::cleanup(&mut app).await;
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let mut app = TestApp::new().await;

    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.post_logout().await;

    let status = response.status().as_u16();
    assert_eq!(status, 401);

    let body: ErrorResponse = response
        .json()
        .await
        .expect("Failed to parse response body");
    assert_eq!(body.error, "Invalid auth token");

    TestApp::cleanup(&mut app).await;
}
