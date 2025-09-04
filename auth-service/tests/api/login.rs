use crate::helpers::{get_random_email, TestApp};

// Tokio's test macro is used to run the test in an async environment
#[tokio::test]
// async fn login_returns_auth_ui() {
//     let app = TestApp::new().await;
//     let response = app.post_login().await;
//
//     assert_eq!(response.status().as_u16(), 200);
// }
async fn should_return_422_if_malformed_credentials() {
    // Arrange
    let app = TestApp::new().await;
    // Act
    // let response = app
    //     .post_login(&serde_json::json!({"email": "testuser@example.com", "password": 123456}));

    let random_email = get_random_email(); // Call helper method to generate email

    // let test_case =
    //   serde_json::json!({
    //       "email": random_email,
    //       "password": "password123",
    //       "requires2FA": true
    // });

    let body = serde_json::json!({
        "email": random_email,
        // "password": "password".to_string(),
        // "password": test_case["password"],
    });

    // call `post_signup` to add the test user
    // let response = app.post_signup(&test_case).await;
    // assert_eq!(response.status().as_u16(), 201);

    let response = app.post_login(&body).await;
    dbg!("Response: {:?}", &response);
    assert_eq!(response.status().as_u16(), 422);

    let body = serde_json::json!({
        "password": "password".to_string(),
    });

    let response = app.post_login(&body).await;
    dbg!("Response: {:?}", &response);
    assert_eq!(response.status().as_u16(), 422);
}
