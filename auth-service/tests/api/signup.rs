use auth_service::routes::SignupResponse;

use crate::helpers::{get_random_email, TestApp};

// Tokio's test macro is used to run the test in an async environment
#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email(); // Call helper method to generate email

    // add some malformed input test cases
    let test_cases = [
        serde_json::json!({             // no email field
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({             // no password field
            "email": random_email,
            "requires2FA": false
        }),
        serde_json::json!({             // no requires2FA field
            "email": random_email,
            "password": "password123",
        }),
        serde_json::json!({             // string instead of boolean value in requires2FA field
            "email": random_email,
            "password": "password123",
            "requires2FA": "true"
        }),
        serde_json::json!({             // empty JSON object
        }),
    ];

    for test_case in test_cases.iter() {
        // call `post_signup`
        let response = app.post_signup(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email(); // Call helper method to generate email

    let test_case =
        serde_json::json!({
            "email": random_email,
            "password": "password123",
            "requires2FA": true
    });

    // call `post_signup`
    let response = app.post_signup(&test_case).await;
    assert_eq!(response.status().as_u16(), 201);

    let expected_response = SignupResponse {
      message: format!("User {} created successfully", random_email)
    };

    assert_eq!(
        response
        .json::<SignupResponse>()
        .await
        .expect("Failed to parse response body."),
        expected_response
    );


}
