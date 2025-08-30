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

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    // The signup route should return a 400 HTTP status code if an invalid input is sent.
    // The input is considered invalid if:
    // - The email is empty or does not contain '@'
    // - The password is less than 8 characters

    // Create an array of invalid inputs. Then, iterate through the array and
    // make HTTP calls to the signup route. Assert a 400 HTTP status code is returned.
    todo!();
    let _app = TestApp::new().await;

    let random_email = get_random_email(); // Call helper method to generate email

    // add some malformed input test cases
    let _test_cases = [
        serde_json::json!({             // empty email field
            "email": "",
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({             // empty password field
            "email": random_email,
            "password": "",
            "requires2FA": false
        }),
        serde_json::json!({             // empty email & password fields
            "email": "",
            "password": "",
            "requires2FA": false
        }),
        serde_json::json!({             // invalid email
            "email": "random_email",
            "password": "password123",
            "requires2FA": false
        }),
        serde_json::json!({             // invalid password
            "email": random_email,
            "password": "short",
            "requires2FA": true
        }),
        serde_json::json!({             // empty JSON object
        }),
    ];

    // for test_case in test_cases.iter() {
    //     // call `post_signup`
    //     let response = app.post_signup(test_case).await;
    //     assert_eq!(response.status().as_u16(), 400, "Failed for input: {:?}", test_case);
    //
    //     assert_eq!(
    //         response
    //             .json::<ErrorResponse>()
    //             .await
    //             .expect("Failed to deserialize response body to ErrorResponse")
    //             .error,
    //         "Invalid credentials".to_owned()
    //     );
    // }
}

#[tokio::test]
async fn should_return_409_if_email_already_exists() {
    // Call the signup route twice. The second request should fail with a 409 HTTP status code
    todo!()
}

