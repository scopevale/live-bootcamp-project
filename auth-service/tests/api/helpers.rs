use std::sync::Arc;
use tokio::sync::RwLock;

use auth_service::{
    domain::AppState, services::hashmap_user_store::HashmapUserStore, Application,
};

use reqwest::cookie::Jar;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub cookie_jar: Arc<Jar>,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
        let app_state = AppState::new(user_store);

        let app = Application::build(app_state, "127.0.0.1:0")
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        // Run the auth service in a separate async task
        // to avoid blocking the main test thread.
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        // Create a Reqwest http client instance
        // let http_client = reqwest::Client::new();

        // Create a Reqwest HTTP client with cookie store enabled
        let cookie_jar = Arc::new(Jar::default());
        let http_client = reqwest::Client::builder()
            // .cookie_provider(Arc::clone(&cookie_jar))
            .cookie_provider(cookie_jar.clone())
            .build()
            .expect("Failed to build HTTP client with cookie jar.");

        // Create new `TestApp` instance and return it
        Self {
            address,
            cookie_jar,
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
        Body: serde::Serialize
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

    pub async fn post_verify_2fa(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/verify-2fa", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_token(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/verify-token", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}
