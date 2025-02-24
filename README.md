# Rust Actix Clean Architecture Starter

A production-ready starter template for building REST APIs with Rust, following clean architecture principles.

## Features

- 🏗️ **Clean Architecture** - Organized in layers (Domain, Application, Infrastructure, Interfaces)
- 🚀 **Actix-web** - High-performance, async web framework
- 📚 **SQLx** - Async PostgreSQL integration with compile-time query checking
- 📝 **OpenAPI/Swagger** - Automatic API documentation with utoipa
- 🔒 **Environment Configuration** - Using dotenv for flexible configuration
- 🧪 **Development Tools** - Formatting, linting, and git hooks
- 🔄 **GitHub Automation** - Workflows for CI, issue management, and PRs

## Prerequisites

- Rust (latest stable)
- PostgreSQL
- Node.js (for development tools)
- pnpm (package manager)

## Project Structure

The project follows clean architecture principles, organized in layers:

```
src/
├── domain/                # Enterprise business rules
│   ├── entities/          # Business objects and DTOs
│   │   ├── product.rs     # Product entity and DTOs
│   │   └── user.rs        # User entity and DTOs
│   └── repositories/      # Repository interfaces
│
├── application/          # Application business rules
│   ├── error.rs          # Application-specific errors
│   └── use_cases/        # Business use cases
│       ├── product/      # Product-related use cases
│       │   ├── create_product.rs
│       │   ├── delete_product.rs
│       │   ├── get_product.rs
│       │   ├── list_products.rs
│       │   └── update_product.rs
│       └── user/         # User-related use cases
│           ├── create_user.rs
│           ├── delete_user.rs
│           ├── get_user.rs
│           ├── list_users.rs
│           └── update_user.rs
│
├── infrastructure/       # External implementations
│   ├── repositories/     # Repository implementations
│   │   ├── product.rs    # Product repository PostgreSQL impl
│   │   └── user.rs       # User repository PostgreSQL impl
│   └── database/         # Database-specific code
│       └── migrations/   # SQL migrations
│
├── interfaces/          # Interface adapters
│   └── api/             # REST API
│       ├── controllers/ # Request handlers
│       ├── middleware/  # API middleware
│       ├── routes.rs    # Route definitions
│       └── docs.rs      # OpenAPI documentation
│
└── config/             # Configuration
    ├── app.rs          # Application configuration
    ├── database.rs     # Database configuration
    ├── environment.rs  # Environment variables
    └── logger.rs       # Logging configuration
```

### Layer Responsibilities

#### Domain Layer (`domain/`)

- Contains enterprise-wide business rules
- Defines entities and repository interfaces
- No dependencies on other layers
- Pure business logic

#### Application Layer (`application/`)

- Contains application-specific business rules
- Implements use cases using domain entities
- Depends only on domain layer
- Orchestrates domain objects

#### Infrastructure Layer (`infrastructure/`)

- Implements interfaces defined in domain layer
- Contains database implementations
- Handles external concerns (database, external services)
- Depends on domain and application layers

#### Interfaces Layer (`interfaces/`)

- Contains controllers and presenters
- Handles HTTP requests and responses
- Implements API endpoints
- Uses application use cases

#### Configuration (`config/`)

- Manages application configuration
- Handles environment variables
- Sets up database connections
- Configures logging

### Key Components

#### Entities

- `Product`: Represents product data and operations
- `User`: Represents user data and operations

#### Use Cases

- Product Management:
  - Create, Read, Update, Delete operations
  - List all products
  - Input validation and business rules
- User Management:
  - User registration and management
  - CRUD operations for users
  - User data validation

#### API Endpoints

- Products API:
  - `POST /api/products` - Create product
  - `GET /api/products` - List products
  - `GET /api/products/{id}` - Get product
  - `PUT /api/products/{id}` - Update product
  - `DELETE /api/products/{id}` - Delete product
- Users API:
  - `POST /api/users` - Create user
  - `GET /api/users` - List users
  - `GET /api/users/{id}` - Get user
  - `PUT /api/users/{id}` - Update user
  - `DELETE /api/users/{id}` - Delete user

#### Database

- PostgreSQL for data persistence
- SQLx for type-safe database operations
- Migration-based schema management

## Getting Started

1. Clone the repository:

```bash
git clone https://github.com/yourusername/rust-actix-clean-starter
cd rust-actix-clean-starter
```

2. Install dependencies:

```bash
# Install Rust dependencies

cargo fetch

# Install development tools

pnpm install

```

3. Set up environment:

````bash
# Copy example environment file

cp .env.example .env

# Edit .env with your configuration

```bash

4. Set up database:

```bash

# Make the setup script executable

chmod +x scripts/setup_db.sh

# Run database setup

./scripts/setup_db.sh

````

5. Run the application:

```bash

cargo run

```

The API will be available at `http://localhost:8080`
Swagger UI at `http://localhost:8080/swagger-ui/`

## Development

### Code Style and Formatting

We use rustfmt and prettier for code formatting:

```bash

# Format Rust code

cargo fmt

# Format other files

pnpm fmt

```

### Git Hooks

This project uses husky for git hooks:

- **pre-commit**: Formats code and runs clippy
- **commit-msg**: Validates commit message format
- **pre-push**: Runs build and clippy checks
- **post-merge**: Updates dependencies

### Commit Messages

We follow conventional commits specification:

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Formatting
- `refactor`: Code restructuring
- `perf`: Performance improvement
- `test`: Adding tests
- `chore`: Maintenance

Example:

```

git commit -m "feat(auth): add user authentication endpoint"

```

### GitHub Workflows

- **Rust CI**: Builds and checks code on push/PR
- **Issue Branch**: Creates branches automatically when issues are assigned
- **Issue Autolink**: Links PRs to issues automatically

### Environment Variables

| Variable     | Description               | Default                                                   |
| ------------ | ------------------------- | --------------------------------------------------------- |
| SERVER_HOST  | Server host address       | 127.0.0.1                                                 |
| SERVER_PORT  | Server port               | 8080                                                      |
| DATABASE_URL | PostgreSQL connection URL | postgres://postgres:postgres@localhost:5432/rust_clean_db |
| RUST_LOG     | Logging level             | debug                                                     |

## API Documentation

API documentation is available through Swagger UI at `/swagger-ui/` when the application is running.

## Testing

```bash

# Run all tests

cargo test

# Run specific test

cargo test test_name

```

## Contributing

1. Create an issue (or pick an existing one)
2. Get assigned to the issue (branch will be created automatically)
3. Make your changes
4. Create a PR (will be linked to issue automatically)
5. Wait for review and CI checks

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [Actix Web](https://actix.rs/)
- [SQLx](https://github.com/launchbadge/sqlx)
- [Utoipa](https://github.com/juhaku/utoipa)
