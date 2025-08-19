use crate::helpers::TestApp;

// Tokio's test macro is used to run the test in an async environment
#[tokio::test]
async fn verify_token_returns_auth_ui() {
    let app = TestApp::new().await;
    let response = app.post_verify_token().await;

    assert_eq!(response.status().as_u16(), 200);
}
