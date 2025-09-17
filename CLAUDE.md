# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Architecture

This is a microservices-based authentication system built in Rust with two main services:

- **auth-service** (Port 3000): Authentication service handling user registration, login, 2FA verification, and token validation
- **app-service** (Port 8000): Web application service that serves the frontend and communicates with auth-service for protected routes

### Key Architecture Components

**Auth Service** (`auth-service/`):
- Domain-driven design with modules: `domain/`, `routes/`, `services/`, `utils/`
- Uses Axum web framework with JWT-based authentication
- In-memory user store (`HashmapUserStore`) for development
- CORS configuration for cross-origin requests from app-service
- Routes: `/signup`, `/login`, `/logout`, `/verify-2fa`, `/verify-token`

**App Service** (`app-service/`):
- Simple Axum server serving static assets and HTML templates (Askama)
- Protected route that validates JWT tokens via auth-service API calls
- Cookie-based session management

## Development Commands

### Building
```bash
# Build both services
cargo install cargo-watch
cd auth-service && cargo build
cd ../app-service && cargo build
```

### Running Services (Manual)
```bash
# Terminal 1 - Auth service (localhost:3000)
cd auth-service
cargo watch -q -c -w src/ -w assets/ -x run

# Terminal 2 - App service (localhost:8000)  
cd app-service
cargo watch -q -c -w src/ -w assets/ -w templates/ -x run
```

### Running Services (Docker)
```bash
./docker.sh  # Loads .env and runs podman compose
```

### Testing
```bash
# Run tests for auth-service
cd auth-service
cargo test

# Run specific test
cargo test test_name
```

## Environment Setup

Auth service requires `.env` file in `auth-service/` directory with `JWT_SECRET` variable. The `docker.sh` script automatically loads environment variables from this file.

## Inter-Service Communication

App service communicates with auth service via HTTP API calls. The `/protected` route in app-service validates JWT tokens by making POST requests to auth-service's `/verify-token` endpoint.

Environment variables control service hostnames:
- `AUTH_SERVICE_IP`: Used by app-service for auth-service links (default: localhost)
- `AUTH_SERVICE_HOST_NAME`: Used by app-service for API calls (default: 0.0.0.0)