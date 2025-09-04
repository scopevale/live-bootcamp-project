use crate::helpers::{get_random_email, TestApp};

// Tokio's test macro is used to run the test in an async environment
#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    let random_email = get_random_email(); // Call helper method to generate email

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
    }
}

#[tokio::test]
async fn should_return_400_if_password_incorrect() {
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
        serde_json::json!({             // bad email field
            "email": random_email,
            "password": "password1234",
        }),
         serde_json::json!({             // no email field
            "email": random_email,
            "password": "pass123",
        }),
        serde_json::json!({             // empty password field
            "email": random_email,
            "password": "",
        }),
        // serde_json::json!({             // empty email field
        //     "email": random_email,
        //     "password": "password123",
        // }),
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
    }
}
