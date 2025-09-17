# Auth Service

## Project Overview

The `auth-service` is a Rust-based web service that provides a JWT-based authentication system. It is built with the Axum framework and is designed to be run in a Docker container. The service exposes endpoints for user signup, login, logout, and token verification.

## API Endpoints

The following are the API endpoints provided by the service:

### `GET /`

*   **Description**: Serves a basic login/signup UI.
*   **Responses**:
    *   `200 OK`: Returns the HTML for the login/signup page.

### `POST /signup`

*   **Description**: Registers a new user.
*   **Request Body**:
    ```json
    {
        "email": "user@example.com",
        "password": "password123",
        "requires2FA": false
    }
    ```
*   **Responses**:
    *   `201 Created`: If the user is created successfully.
    *   `409 Conflict`: If a user with the given email already exists.
    *   `422 Unprocessable Entity`: If the email or password format is invalid.

### `POST /login`

*   **Description**: Authenticates a user and returns a JWT as an `HttpOnly` cookie.
*   **Request Body**:
    ```json
    {
        "email": "user@example.com",
        "password": "password123"
    }
    ```
*   **Responses**:
    *   `200 OK`: If the login is successful. The response will include a `Set-Cookie` header with the JWT.
    *   `401 Unauthorized`: If the credentials are incorrect.
    *   `404 Not Found`: If the user does not exist.

### `POST /logout`

*   **Description**: Logs out a user by invalidating their JWT.
*   **Responses**:
    *   `200 OK`: If the logout is successful. The response will include a `Set-Cookie` header to clear the JWT cookie.
    *   `401 Unauthorized`: If the JWT is missing or invalid.

### `POST /verify-token`

*   **Description**: Verifies the validity of a JWT.
*   **Request Body**:
    ```json
    {
        "token": "your_jwt_token"
    }
    ```
*   **Responses**:
    *   `200 OK`: If the token is valid.
    *   `401 Unauthorized`: If the token is invalid.

### `POST /verify-2fa`

*   **Description**: This endpoint is currently a placeholder and does not contain any logic. It will return a `200 OK` response for any request.

## Authentication

The service uses JWT-based authentication. When a user successfully logs in, a JWT is generated and set as an `HttpOnly` cookie named `jwt`. The token is signed with a secret and has an expiration time of 10 minutes.

The JWT contains the following claims:

*   `sub`: The user's email address.
*   `exp`: The expiration time of the token.

The `/verify-token` endpoint can be used to validate a JWT. The `/logout` endpoint invalidates a JWT by adding it to a denylist of banned tokens.

## Data Storage

The service uses in-memory data stores for users and banned tokens. This means that all data will be lost when the service restarts.

*   **User Store**: A `HashMap` is used to store users, with the user's email as the key.
*   **Banned Token Store**: A `HashSet` is used to store banned JWTs.

This implementation is suitable for development and testing, but it should be replaced with a persistent data store for a production environment.

## Dependencies

The project uses the following key dependencies:

*   [axum](https://github.com/tokio-rs/axum): A web application framework for Rust.
*   [tokio](https://github.com/tokio-rs/tokio): An asynchronous runtime for Rust.
*   [serde](https://github.com/serde-rs/serde): A framework for serializing and deserializing Rust data structures.
*   [jsonwebtoken](https://github.com/Keats/jsonwebtoken): A library for creating and validating JWTs.
*   [chrono](https://github.com/chronotope/chrono): A library for date and time handling.
*   [uuid](https://github.com/uuid-rs/uuid): A library for generating and parsing UUIDs.

## How to Run

The service can be built and run using Docker and Docker Compose.

1.  **Build the Docker image**:
    ```bash
    docker-compose build
    ```
2.  **Run the service**:
    ```bash
    docker-compose up
    ```

The service will be available at `http://localhost:3000`.
