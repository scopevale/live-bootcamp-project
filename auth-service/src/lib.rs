use axum::{
    body::Body,
    http::header,
    response::{IntoResponse, Response},
    routing::post,
    serve::Serve,
    Router
};
use tower_http::services::ServeDir;
use std::error::Error;

pub mod routes;
pub mod domain;
pub mod services;

use routes::{signup, login, logout, verify_2fa, verify_token};

// This struct encapsulates our application-related logic.
pub struct Application {
    server: Serve<Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        let router = Router::new()
        .nest_service("/", ServeDir::new("assets"))
        .route("/signup", post(signup).options(options_handler))
        .route("/login", post(login).options(options_handler))
        .route("/logout", post(logout).options(options_handler))
        .route("/verify-2fa", post(verify_2fa).options(options_handler))
        .route("/verify-token", post(verify_token).options(options_handler));

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
        .body(Body::empty())
        .unwrap()
}
