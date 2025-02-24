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

```
domain/ # Business entities and interfaces
application/ # Use cases and business logic
infrastructure/ # External implementations (database, etc.)
interfaces/ # API controllers and middleware
config/ # Application configuration
```

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

This project is licensed under the ISC License - see the LICENSE file for details.

## Acknowledgments

- [Actix Web](https://actix.rs/)
- [SQLx](https://github.com/launchbadge/sqlx)
- [Utoipa](https://github.com/juhaku/utoipa)
