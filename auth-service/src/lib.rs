use axum::{
    body::Body,
    http::{header, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
    serve::Serve,
    Json, Router,
};
use redis::{Client, RedisResult};
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::error::Error;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};

pub mod domain;
pub mod routes;
pub mod services;
pub mod utils;

use domain::{AppState, AuthAPIError};
use routes::{login, logout, signup, verify_2fa, verify_token};
use utils::tracing::{make_span_with_request_id, on_request, on_response};

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
            // Allow the specified origins
            .allow_origin(allowed_origins);

        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(signup).options(options_handler))
            .route("/login", post(login).options(options_handler))
            .route("/logout", post(logout).options(options_handler))
            .route("/verify-2fa", post(verify_2fa).options(options_handler))
            .route("/verify-token", post(verify_token).options(options_handler))
            .with_state(app_state)
            .layer(cors)
            .layer(
                // Add a TraceLayer for HTTP requests to enable detailed tracing
                // This layer will create spans for each request using the make_span_with_request_id function,
                // and log events at the start and end of each request using on_request and on_response functions.
                TraceLayer::new_for_http()
                    .make_span_with(make_span_with_request_id)
                    .on_request(on_request)
                    .on_response(on_response),
            );

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // Create a new Application instance and return it
        Ok(Application { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        tracing::info!("listening on {}", &self.address);
        self.server.await
    }
}

async fn options_handler() -> impl IntoResponse {
    Response::builder()
        .status(200)
        .header(header::ALLOW, "POST, OPTIONS")
        .body(Body::empty())
        .unwrap()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        log_error_chain(&self);
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::UserNotFound => (StatusCode::NOT_FOUND, "User not found"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::IncorrectCredentials => {
                (StatusCode::UNAUTHORIZED, "Incorrect credentials")
            }
            AuthAPIError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid auth token"),
            AuthAPIError::MissingToken => (StatusCode::BAD_REQUEST, "Missing auth token"),
            AuthAPIError::TokenAlreadyBanned => (StatusCode::CONFLICT, "Token already banned"),
            AuthAPIError::TokenBanFailed => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Failed to ban token")
            }
            AuthAPIError::UnexpectedError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
        };
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
        (status, body).into_response()
    }
}

fn log_error_chain(e: &(dyn Error + 'static)) {
    let separator =
        "\n-----------------------------------------------------------------------------------\n";
    let mut report = format!("{}{:?}\n", separator, e);
    let mut current = e.source();
    while let Some(cause) = current {
        let str = format!("Caused by:\n\n{:?}", cause);
        report = format!("{}\n{}", report, str);
        current = cause.source();
    }
    report = format!("{}\n{}", report, separator);
    tracing::error!("{}", report);
}

// fn log_error_chain(e: &(dyn std::error::Error + 'static)) {
//     let separator =
//         "\n-----------------------------------------------------------------------------------\n";
//
//     // Use Display `{}` instead of Debug `{:?}` so ESC bytes aren't escaped as `\\x1b`
//     let mut report = format!("{separator}{e}\n");
//
//     let mut current = e.source();
//     while let Some(cause) = current {
//         let str = format!("Caused by:{}\n\n", cause);
//         report = format!("{}\n{}", report, str);
//         current = cause.source();
//     }
//
//     report = format!("{}\n{}", report, separator);
//     tracing::error!("{}", report);
// }

pub async fn get_postgres_pool(url: &Secret<String>) -> Result<PgPool, sqlx::Error> {
    // Create a new PostgreSQL connection pool
    PgPoolOptions::new()
        .max_connections(5)
        .connect(url.expose_secret())
        .await
}

pub fn get_redis_client(redis_hostname: String) -> RedisResult<Client> {
    let redis_url = format!("redis://{}/", redis_hostname);
    redis::Client::open(redis_url)
}

#[cfg(test)]
pub mod test_helpers;
