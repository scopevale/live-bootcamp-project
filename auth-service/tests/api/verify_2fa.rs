use auth_service::domain::LoginAttemptId;

use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;
    let random_email = get_random_email(); // Call helper method to generate email
    let login_attempt_id = LoginAttemptId::default();
    let expected_error_message =
        "error decoding response body: expected value at line 1 column 1".to_owned();

    let test_bodies = [
        serde_json::json!({
            "loginAttemptId": login_attempt_id,
        }),
        serde_json::json!({
            "2FACode": "123456",
        }),
        serde_json::json!({
            "email": random_email,
        }),
        serde_json::json!({
            "loginAttemptId": login_attempt_id,
            "2FACode": "123456",
        }),
        serde_json::json!({
            "loginAttemptId": login_attempt_id,
            "email": random_email,
        }),
        serde_json::json!({
            "2FACode": "123456",
            "email": random_email,
        }),
        serde_json::json!({
            "bogusField": "bogusValue",
        }),
        serde_json::json!({ // empty body
        }),
    ];

    for body in test_bodies.iter() {
        // call `post_verify_2fa`
        let response = app.post_verify_2fa(&body).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            body
        );
        let req_response = response.json::<serde_json::Value>().await;
        assert_eq!(
            req_response.err().unwrap().to_string(),
            expected_error_message
        );
    }
}
