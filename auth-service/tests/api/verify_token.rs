use crate::helpers::TestApp;

// Tokio's test macro is used to run the test in an async environment
// #[tokio::test]
// async fn verify_token_returns_auth_ui() {
//     let app = TestApp::new().await;
//     let response = app.post_verify_token().await;
//
//     assert_eq!(response.status().as_u16(), 200);
// }

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let test_cases = vec![
        serde_json::json!({
            "token": true,
        }),
        serde_json::json!({}),
    ];

    for test_case in test_cases {
        let response = app.post_verify_token(&test_case).await;
        assert_eq!(response.status().as_u16(), 422);
    }
}
