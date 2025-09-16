use auth_service::domain::LoginAttemptId;

use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;
    let expected_error_message =
        "error decoding response body: expected value at line 1 column 1".to_owned();

    let test_bodies = [
        serde_json::json!({             // no code field
            "loginAttemptId": LoginAttemptId::default(),
        }),
        serde_json::json!({             // no loginAttemptId field
            "code": "123456",
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
