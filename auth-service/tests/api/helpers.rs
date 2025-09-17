use std::sync::Arc;
use tokio::sync::RwLock;

use auth_service::{
    domain::{AppState, BannedTokenStoreType, TwoFACodeStoreType},
    services::{HashmapTwoFACodeStore, HashmapUserStore, HashsetBannedTokenStore, MockEmailClient},
    utils::constants::test,
    Application,
};

use reqwest::cookie::Jar;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TestApp {
    pub address: String,
    pub cookie_jar: Arc<Jar>,
    pub banned_token_store: BannedTokenStoreType,
    pub two_fa_code_store: TwoFACodeStoreType,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
        let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
        let two_fa_code_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));

        let email_client = Arc::new(MockEmailClient);

        let app_state = AppState::new(
            user_store,
            banned_token_store.clone(),
            two_fa_code_store.clone(),
            email_client,
        );

        println!("App state: {:?}", &app_state);

        let app = Application::build(app_state, test::APP_ADDRESS)
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        // Run the auth service in a separate async task
        // to avoid blocking the main test thread.
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        // Create a Reqwest HTTP client with cookie store enabled
        let cookie_jar = Arc::new(Jar::default());
        let http_client = reqwest::Client::builder()
            .cookie_provider(cookie_jar.clone())
            .build()
            .expect("Failed to build HTTP client with cookie jar.");

        // Create new `TestApp` instance and return it
        Self {
            address,
            cookie_jar,
            banned_token_store,
            two_fa_code_store,
            http_client,
        }
    }

    // TODO: Fix this function to properly return the user ID from the signup response
    // pub async fn create_user_and_login(&self) -> (reqwest::cookie::Cookie, Uuid) {
    //     let email = get_random_email();
    //     let password = "password123";
    //     let signup_body = serde_json::json!({
    //         "email": email,
    //         "password": password
    //     });
    //     let signup_response = self.post_signup(&signup_body).await;
    //     assert_eq!(signup_response.status().as_u16(), 201);
    //     let login_body = serde_json::json!({
    //         "email": email,
    //         "password": password
    //     });
    //     let login_response = self.post_login(&login_body).await;
    //     assert_eq!(login_response.status().as_u16(), 200);
    //     // Extract the JWT cookie from the cookie jar
    //     let url = reqwest::Url::parse(&self.address).expect("Failed to parse URL");
    //     let cookies = self.cookie_jar.cookies(&url).expect("No cookies found.");
    //     let cookie_str = cookies.to_str().expect("Failed to convert cookies to string.");
    //     // Find the JWT cookie in the cookie string
    //     let jwt_cookie = cookie_str
    //         .split(';')
    //         .find(|c| c.trim_start().starts_with("jwt="))
    //         .expect("JWT cookie not found.")
    //         .trim_start()
    //         .to_string();
    //     // Parse the cookie string into a `reqwest::cookie::Cookie`
    //     let cookie = reqwest::cookie::Cookie::parse(jwt_cookie)
    //         .expect("Failed to parse JWT cookie.");
    //     // For simplicity, we'll return a dummy user ID (UUID)
    //     let user_id = Uuid::new_v4();
    //     (cookie, user_id)
    // }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_logout(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_2fa<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/verify-2fa", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_token<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/verify-token", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}
