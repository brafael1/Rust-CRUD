# Rust CRUD API

High-performance REST API built with Rust, Axum, PostgreSQL, and Redis.

## Features

- **CRUD Operations**: Create, Read, Update, Delete users
- **Authentication**: JWT-based authentication with Argon2 password hashing
- **Caching**: Redis cache for improved read performance
- **Pagination**: Cursor-based pagination (no OFFSET)
- **Observability**: Structured logging with tracing
- **Metrics**: Prometheus-compatible /metrics endpoint

## Tech Stack

- **Language**: Rust 1.75+
- **Web Framework**: Axum 0.7+
- **Database**: PostgreSQL 15+
- **Cache**: Redis 7+
- **ORM**: SQLx
- **Auth**: JWT + Argon2

## Getting Started

### Prerequisites

- Rust 1.75+
- PostgreSQL 15+
- Redis 7+
- Docker & Docker Compose (optional)

### Running Locally

1. Clone the repository
2. Set up environment variables or use defaults:

```bash
export APP_DATABASE__HOST=localhost
export APP_DATABASE__PORT=5432
export APP_DATABASE__USERNAME=postgres
export APP_DATABASE__PASSWORD=postgres
export APP_DATABASE__NAME=rust_crud
export APP_REDIS__HOST=localhost
export APP_REDIS__PORT=6379
export APP_JWT__SECRET=your-secret-key
```

3. Run the server:

```bash
cargo run --release
```

### Using Docker

```bash
docker-compose up --build
```

## API Endpoints

### Public Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | /health | Health check |
| POST | /auth/login | User login |

### Protected Endpoints

All `/users/*` endpoints require authentication (except documented below).

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | /users | Create a new user (public registration) |
| GET | /users | List users with cursor-based pagination |
| GET | /users/:id | Get user by ID |
| PUT | /users/:id | Update user |
| DELETE | /users/:id | Delete user |

## Request Examples

### Health Check
```bash
curl http://localhost:8080/health
```

### User Registration
```bash
curl -X POST http://localhost:8080/users \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","email":"admin@test.com","password":"password123"}'
```

### User Login
```bash
curl -X POST http://localhost:8080/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"admin@test.com","password":"password123"}'
```

### List Users
```bash
curl http://localhost:8080/users
```

### Get User by ID
```bash
curl http://localhost:8080/users/USER_ID
```

### Update User
```bash
curl -X PUT http://localhost:8080/users/USER_ID \
  -H "Content-Type: application/json" \
  -d '{"email":"newemail@test.com"}'
```

### Delete User
```bash
curl -X DELETE http://localhost:8080/users/USER_ID
```

## Configuration

Configuration can be provided via environment variables with prefix `APP_`:

| Variable | Default | Description |
|----------|---------|-------------|
| APP_SERVER__PORT | 8080 | Server port |
| APP_SERVER__HOST | 0.0.0.0 | Server host |
| APP_DATABASE__HOST | localhost | PostgreSQL host |
| APP_DATABASE__PORT | 5432 | PostgreSQL port |
| APP_DATABASE__USERNAME | postgres | PostgreSQL username |
| APP_DATABASE__PASSWORD | postgres | PostgreSQL password |
| APP_DATABASE__NAME | rust_crud | Database name |
| APP_REDIS__HOST | localhost | Redis host |
| APP_REDIS__PORT | 6379 | Redis port |
| APP_JWT__SECRET | - | JWT secret (required) |

## Architecture

The project follows a layered architecture:

```
src/
├── handlers/      # HTTP handlers (controllers)
├── services/     # Business logic
├── db/          # Database layer (SQLx)
├── cache/       # Redis cache layer
├── models/      # Data models
├── config/     # Configuration
├── errors/     # Error types
├── middlewares/ # HTTP middleware
└── utils/      # Utilities
```

## Performance Considerations

- Connection pooling for PostgreSQL
- Cursor-based pagination (avoids OFFSET performance issues)
- Redis caching for read-heavy operations
- Prepared statements via SQLx
- Async/await throughout

## Benchmarking

See the `benchmarks/` directory for k6 load test scripts:

```bash
k6 run benchmarks/users.js
```

## License

MIT