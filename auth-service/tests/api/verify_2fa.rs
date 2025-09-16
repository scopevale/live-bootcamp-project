use crate::helpers::TestApp;

// Tokio's test macro is used to run the test in an async environment
#[tokio::test]
async fn verify_2fa_returns_auth_ui() {
    let app = TestApp::new().await;
    let test_body = serde_json::json!({
        "user_id": uuid::Uuid::new_v4(),
        "code": "123456"
    });
    let response = app.post_verify_2fa(&test_body).await;

    assert_eq!(response.status().as_u16(), 200);
}
