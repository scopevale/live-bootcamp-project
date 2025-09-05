use auth_service::{routes::LoginResponse, ErrorResponse};
use crate::helpers::{get_random_email, TestApp};

// Tokio's test macro is used to run the test in an async environment
#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    let random_email = get_random_email(); // Call helper method to generate email

    let expected_error_message = "error decoding response body: expected value at line 1 column 1".to_owned();

    let test_bodies = [
        serde_json::json!({             // no password field
            "email": random_email,
        }),
         serde_json::json!({             // no email field
            "password": "password123",
        }),
    ];

    for body in test_bodies.iter() {
        // call `post_login`
        let response = app.post_login(&body).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            body
        );

        let req_response = response
            .json::<ErrorResponse>()
            .await;

        assert_eq!(
            req_response.err().unwrap().to_string(),
            expected_error_message
        );
    }
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email(); // Call helper method to generate email

    // add a new user to test against
    let user = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false,
    });

    let response = app.post_signup(&user).await;
    assert_eq!(response.status().as_u16(), 201);

    let test_bodies = [
        serde_json::json!({             // password too short
            "email": random_email,
            "password": "pass123",
        }),
        serde_json::json!({             // empty password field
            "email": random_email,
            "password": "",
        }),
    ];

    for body in test_bodies.iter() {
        // call `post_login`
        let response = app.post_login(&body).await;
        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            body
        );
       assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid credentials".to_owned()
        );
    }
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;
    let random_email = get_random_email(); // Call helper method to generate email

    // add a new user to test against
    let user = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false,
    });

    let response = app.post_signup(&user).await;
    assert_eq!(response.status().as_u16(), 201);

    let test_bodies = [
        serde_json::json!({             // valid but incorrect password
            "email": random_email,
            "password": "password1234",
        }),
        serde_json::json!({             // valid but incorrect email
            "email": "me@example.com",
            "password": "password123",
        }),
];
    for body in test_bodies.iter() {
        let response = app.post_login(&body).await;
        assert_eq!(
            response.status().as_u16(),
            401,
            "Failed for input: {:?}",
            body
        );
       assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Incorrect credentials".to_owned()
        );
    }
}

// #[tokio::test]
// async fn should_login_successfully_with_valid_credentials() {
//     let app = TestApp::new().await;
//     let random_email = get_random_email(); // Call helper method to generate email
//     // add a new user to test against
//     let user = serde_json::json!({
//         "email": random_email,
//         "password": "password123",
//         "requires2FA": false,
//     });
//     let response = app.post_signup(&user).await;
//     assert_eq!(response.status().as_u16(), 201);
//     // now try to login with the same credentials
//     let login_request = serde_json::json!({
//         "email": random_email,
//         "password": "password123",
//     });
//     let response = app.post_login(&login_request).await;
//     assert_eq!(response.status().as_u16(), 200);
//     let expected_response = LoginResponse {
//         message: format!("User {} logged in successfully", random_email),
//     };
//     let response_body = response
//         .json::<LoginResponse>()
//         .await
//         .expect("Could not deserialize response body to LoginResponse");
//     assert_eq!(response_body, expected_response);
// }
