# Contributing to areuok-server

Thank you for your interest in contributing to areuok-server! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Commit Message Guidelines](#commit-message-guidelines)
- [Pull Request Process](#pull-request-process)
- [Testing Guidelines](#testing-guidelines)
- [Documentation](#documentation)

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers and help them learn
- Focus on what is best for the community
- Show empathy towards other community members

## Getting Started

### Prerequisites

- Rust 1.70 or higher
- PostgreSQL 12+
- Docker and Docker Compose (recommended)
- Git

### Setup

1. **Fork and Clone**
   ```bash
   # Fork the repository on GitHub
   git clone https://github.com/YOUR_USERNAME/areuok-server.git
   cd areuok-server
   ```

2. **Install Dependencies**
   ```bash
   cargo build
   ```

3. **Start Development Environment**
   ```bash
   make up
   ```

4. **Run Tests**
   ```bash
   make test
   ```

## Development Workflow

### Branching Strategy

- `main` - Production-ready code
- `develop` - Integration branch for features
- `feature/*` - New features
- `bugfix/*` - Bug fixes
- `hotfix/*` - Critical production fixes

### Workflow Steps

1. **Create Feature Branch**
   ```bash
   git checkout develop
   git pull origin develop
   git checkout -b feature/your-feature-name
   ```

2. **Make Changes**
   - Write code following [Coding Standards](#coding-standards)
   - Add tests for new functionality
   - Update documentation

3. **Test Locally**
   ```bash
   # Run all tests
   make test

   # Run comprehensive lint check
   make lint

   # Or run individual checks
   make fmt-check      # Check formatting
   make clippy        # Run linter
   make check         # Check compilation

   # Install pre-commit hooks (recommended)
   make install-hooks
   ```

### Pre-commit Hooks

We provide pre-commit hooks that automatically check your code before each commit:

```bash
make install-hooks
```

This will run:
- Code formatting check (rustfmt)
- Clippy lints
- Tests (if DATABASE_URL is set)

You can also run pre-commit checks manually:
```bash
make pre-commit
# or
./scripts/pre-commit
```

4. **Commit Changes**
   - Follow [Commit Message Guidelines](#commit-message-guidelines)

5. **Push and Create PR**
   ```bash
   git push origin feature/your-feature-name
   ```
   Then create a Pull Request on GitHub

## Coding Standards

### Rust Style

- Follow official Rust style guidelines: [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for consistent formatting
- Use `cargo clippy` for linting

### Code Organization

- **crates/models/** - Data structures and types
- **crates/db/** - Database operations
- **crates/api/** - HTTP handlers and routing
- **crates/server/** - Application bootstrap

### Error Handling

- Use custom `AppError` enum for API errors
- Provide meaningful error messages
- Log errors appropriately

```rust
#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    Internal(String),
}
```

### Database Operations

- Use sqlx for type-safe queries
- Use prepared statements for all queries
- Handle database errors gracefully

```rust
sqlx::query!(
    r#"SELECT * FROM devices WHERE device_id = $1"#,
    device_id
)
.fetch_optional(&pool)
.await?
.ok_or(AppError::NotFound("Device not found".to_string()))?;
```

### API Design

- RESTful conventions
- Consistent response formats
- Proper HTTP status codes
- Input validation

### Documentation

- Document public APIs with `///` doc comments
- Document complex algorithms
- Keep doc comments up to date

```rust
/// Register a new device with optional IMEI binding.
///
/// # Arguments
///
/// * `device_name` - Name of the device (must be unique)
/// * `imei` - Optional IMEI for device binding
/// * `mode` - Device mode (signin or supervisor)
///
/// # Returns
///
/// Returns `Device` with device_id and other information.
///
/// # Errors
///
/// Returns `AppError::BadRequest` if:
/// - Device name already exists
/// - Invalid device mode
#[tauri::command]
fn register_device(/* ... */) -> Result<Device, String> {
    // ...
}
```

## Commit Message Guidelines

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Type

- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation changes
- `style` - Code style changes (formatting, etc.)
- `refactor` - Code refactoring
- `perf` - Performance improvements
- `test` - Adding or updating tests
- `chore` - Maintenance tasks

### Scope

- `db` - Database changes
- `api` - API changes
- `models` - Data model changes
- `server` - Server configuration
- `docs` - Documentation

### Examples

```
feat(api): add device name update endpoint with 15-day cooldown

- Add PATCH /devices/{id}/name endpoint
- Validate name uniqueness
- Enforce 15-day cooldown period
- Add last_name_updated_at field to devices table

Fixes #123
```

```
fix(db): resolve IMEI uniqueness constraint issue

- Update migration to properly handle NULL values
- Fix UNIQUE constraint on imei column

Closes #456
```

### Rules

- Use imperative mood ("add" not "added")
- Limit first line to 72 characters
- Reference related issues
- Explain what and why, not how

## Pull Request Process

### Before Creating PR

1. **Check Checklist**
   - [ ] Code follows style guidelines
   - [ ] Tests added/updated
   - [ ] Documentation updated
   - [ ] All tests pass
   - [ ] No clippy warnings
   - [ ] Properly formatted

2. **Self-Review**
   ```bash
   # Format check
   cargo fmt --check

   # Lint
   cargo clippy -- -D warnings

   # Tests
   cargo test
   ```

3. **Rebase if Needed**
   ```bash
   git fetch origin
   git rebase origin/develop
   ```

### Creating PR

1. **Title**: Follow commit message format
2. **Description**: Include:
   - What the PR does
   - Why it's needed
   - Breaking changes
   - Screenshots (if UI changes)
   - Related issues

3. **Template**
   ```markdown
   ## Description
   Brief description of changes.

   ## Type of Change
   - [ ] Bug fix
   - [ ] New feature
   - [ ] Breaking change
   - [ ] Documentation update

   ## Testing
   - [ ] Unit tests pass
   - [ ] Integration tests pass
   - [ ] Manually tested

   ## Checklist
   - [ ] Code follows style guidelines
   - [ ] Self-reviewed code
   - [ ] Documentation updated
   - [ ] No new warnings
   - [ ] Tests added for new functionality
   ```

### Review Process

1. **Auto-Checks**: CI/CD runs automatically
2. **Code Review**: At least one approval required
3. **Changes Requested**: Address feedback and update PR
4. **Approval**: PR approved and ready to merge

### After Approval

1. **Squash Commits** (if needed)
   ```bash
   git rebase -i HEAD~n
   ```

2. **Update Branch**
   ```bash
   git checkout develop
   git pull origin develop
   git merge feature/your-feature
   ```

3. **Push**
   ```bash
   git push origin develop
   ```

## Testing Guidelines

### Unit Tests

- Test public functions and methods
- Mock external dependencies
- Cover success and error paths

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_name_validation() {
        // Test cases
        assert!(validate_device_name("").is_err());
        assert!(validate_device_name("Valid Name").is_ok());
    }
}
```

### Integration Tests

- Located in `test/` directory
- Use pytest with Python
- Test API endpoints end-to-end

```python
def test_register_device():
    response = client.post("/devices/register", json={
        "device_name": "Test Device",
        "mode": "signin"
    })
    assert response.status_code == 200
    assert "device_id" in response.json()
```

### Database Tests

- Test migrations
- Test constraints
- Test queries

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_device_registration

# Integration tests
cd test
uv run pytest

# With coverage
cargo tarpaulin --out Html
```

## Documentation

### When to Update Docs

- Adding new features
- Changing API behavior
- Modifying database schema
- Updating dependencies

### Documentation Files

- `README.md` - Project overview
- `CONTRIBUTING.md` - This file
- `docs/api/device-management.md` - API documentation
- `docs/api/overview.md` - API overview
- `docs/database-schema.md` - Database documentation

### Writing Style

- Clear and concise
- Include examples
- Update existing, don't duplicate
- Use markdown formatting

## Quality Assurance

### Pre-Commit Hooks

Consider using pre-commit hooks:
```bash
cargo install pre-commit
pre-commit install
```

### CI/CD

- GitHub Actions runs on every push and PR
- Checks formatting, linting, and tests
- Must pass before merge

### Code Coverage

- Aim for 80%+ coverage
- Add tests for uncovered code
- Review coverage reports

## Getting Help

- **Issues**: Report bugs or request features
- **Discussions**: Ask questions or discuss ideas
- **Pull Requests**: Submit code changes

## Release Process

Only maintainers should handle releases:

1. **Version**: Follow semantic versioning
2. **Changelog**: Update CHANGELOG.md
3. **Tag**: Create git tag
4. **Release**: Create GitHub release

## License

By contributing, you agree that your contributions will be licensed under the same license as the project.

---

**Thank you for contributing to areuok-server!** 🎉
