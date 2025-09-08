use auth_service::{utils::constants::JWT_COOKIE_NAME, ErrorResponse};
use reqwest::Url;

use crate::helpers::TestApp;

// Tokio's test macro is used to run the test in an async environment
#[tokio::test]
async fn logout_returns_auth_ui() {
    let app = TestApp::new().await;
    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    todo!()
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    todo!() // finish implementing this test
}
