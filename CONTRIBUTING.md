# Contributing to Rust Actix Clean Starter

Thank you for your interest in contributing to this project! This document provides guidelines and workflows for contributing.

## Development Process

1. Fork the repository
2. Create an issue for your work
3. Get assigned to the issue
4. Work on your changes in the automatically created branch
5. Submit a pull request

## Setting Up Development Environment

1. Install prerequisites:

   - Rust (latest stable)
   - PostgreSQL
   - Node.js
   - pnpm

2. Clone and setup:

   ```bash
   git clone https://github.com/yourusername/rust-actix-clean-starter
   cd rust-actix-clean-starter
   pnpm install
   cargo fetch
   ```

3. Copy environment file:
   ```bash
   cp .env.example .env
   ```

## Code Style

- Rust code is formatted using `rustfmt`
- Configuration files are formatted using `prettier`
- Maximum line length is 100 characters
- Use 4 spaces for indentation in Rust files
- Use 2 spaces for indentation in other files

## Commit Guidelines

We use conventional commits specification:

- Format: `type(scope): description`
- Types: feat, fix, docs, style, refactor, perf, test, chore
- Scope is optional and should be the module name
- Description should be in present tense

Example:

```bash
git commit -m "feat(auth): add user authentication endpoint"
```

## Pull Request Process

1. Update documentation for any new features
2. Update the README.md if needed
3. Ensure all tests pass
4. Get at least one code review
5. Squash commits if requested

## Testing

- Write unit tests for new features
- Ensure existing tests pass
- Run the full test suite before submitting PR

## Code Review

- All submissions require review
- We use GitHub pull requests for this purpose
- Reviews should focus on:
  - Code correctness
  - Test coverage
  - Documentation
  - Adherence to project style

## Questions or Problems?

- Open an issue for bugs
- Use discussions for questions
- Tag maintainers if urgent

Thank you for contributing!
