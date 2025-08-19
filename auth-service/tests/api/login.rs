use crate::helpers::TestApp;

// Tokio's test macro is used to run the test in an async environment
#[tokio::test]
async fn login_returns_auth_ui() {
    let app = TestApp::new().await;
    let response = app.post_login().await;

    assert_eq!(response.status().as_u16(), 200);
}
