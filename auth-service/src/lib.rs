use axum::{
    body::Body,
    http::{header, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
    serve::Serve,
    Json,
    Router,
};
use tower_http::{cors::CorsLayer, services::ServeDir};
use std::error::Error;
use serde::{Deserialize, Serialize};

pub mod routes;
pub mod domain;
pub mod services;
pub mod utils;

use routes::{signup, login, logout, verify_2fa, verify_token};
use domain::{AppState, AuthAPIError};

// This struct encapsulates our application-related logic.
pub struct Application {
    server: Serve<Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        // Set up CORS layer
        let allowed_origins = vec![
            "http://localhost:8000".parse()?,
            "http://64.227.26.4:8000".parse()?,
        ];

        let cors = CorsLayer::new()
            // Allow GET and POST requests from allowed origins
            .allow_methods([Method::GET, Method::POST])
            // Allow cookies to be sent
            .allow_credentials(true)
            // Allow the specified originsy
            .allow_origin(allowed_origins);

            // .allow_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION])
            // .max_age(std::time::Duration::from_secs(3600));


        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(signup).options(options_handler))
            .route("/login", post(login).options(options_handler))
            .route("/logout", post(logout).options(options_handler))
            .route("/verify-2fa", post(verify_2fa).options(options_handler))
            .route("/verify-token", post(verify_token).options(options_handler))
            .with_state(app_state)
            .layer(cors);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // Create a new Application instance and return it
        Ok(Application { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}

async fn options_handler() -> impl IntoResponse {
    Response::builder()
        .status(200)
        .header(header::ALLOW, "POST, OPTIONS")
        .body(Body::empty()) .unwrap()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::UserNotFound => (StatusCode::NOT_FOUND, "User not found"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::IncorrectCredentials => (StatusCode::UNAUTHORIZED, "Incorrect credentials"),
            AuthAPIError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid auth token"),
            AuthAPIError::MissingToken => (StatusCode::BAD_REQUEST, "Missing auth token"),
            AuthAPIError::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
        };
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
        (status, body).into_response()
    }
}

#[cfg(test)]
pub mod test_helpers;
