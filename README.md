# axum-backend-template

A modern Rust backend template using Axum framework.

## Project Structure

```
axum-backend-template/
├── src/
│   ├── api/            # API routes and controllers
│   ├── common/         # Common utilities
│   ├── config/         # Application configuration
│   ├── domain/         # Business logic and entities
│   ├── infra/          # Infrastructure implementations
│   └── main.rs         # Application entry point
├── .env                # Environment variables
```

## Requirements

- Rust stable (>= 1.86.0 recommended)
- Cargo (included with Rust)

## Dependencies

- axum 0.8.3
- tokio 1.44.2
- tower 0.5.2
- reqwest 0.12.15
- serde 1.0.219
- and more (see Cargo.toml)

## Getting Started

### Setup

```shell script
# Clone the repository
git clone https://github.com/username/axum-backend-template.git
cd axum-backend-template

# Configure environment variables
cp .env.example .env  # and edit as needed
```

### Run

```shell script
# Development mode
cargo build

cargo run

# Production mode
cargo run --release
```

## License

See the LICENSE file for details.