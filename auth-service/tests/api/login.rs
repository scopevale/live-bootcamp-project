use crate::helpers::{get_random_email, TestApp};
use auth_service::{
    domain::Email, routes::TwoFactorAuthResponse, utils::constants::JWT_COOKIE_NAME, ErrorResponse,
};

// Tokio's test macro is used to run the test in an async environment
#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    let random_email = get_random_email(); // Call helper method to generate email

    let expected_error_message =
        "error decoding response body: expected value at line 1 column 1".to_owned();

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

        let req_response = response.json::<ErrorResponse>().await;

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
        serde_json::json!({             // empty password field
            "email": "bad>user@example.com".to_string(),
            "password": "password123",
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
async fn should_return_401_if_incorrect_password() {
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

#[tokio::test]
async fn should_return_404_if_incorrect_username() {
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

    let test_bodies = [serde_json::json!({             // valid but incorrect email
        "email": "me@example.com",
        "password": "password123",
    })];

    for body in test_bodies.iter() {
        let response = app.post_login(&body).await;
        assert_eq!(
            response.status().as_u16(),
            404,
            "Failed for input: {:?}",
            body
        );
        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "User not found".to_owned()
        );
    }
}

#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}

#[tokio::test]
async fn should_return_206_if_valid_credentials_and_2fa_enabled() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);

    let json_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert!(!json_body.login_attempt_id.is_empty());
    assert_eq!(json_body.message, "2FA required".to_owned());

    // assert that 'json_body.login_attempt_id' is stored in the TwoFA code store
    let two_fa_code_store = app.two_fa_code_store.read().await;

    let code_tuple = two_fa_code_store
        .get_code(&Email::parse(random_email).unwrap())
        .await
        .expect("Failed to get 2FA code");

    assert_eq!(code_tuple.0.as_ref(), json_body.login_attempt_id);
}
