# Contributing to OLA

Thank you for your interest in contributing! This document provides guidelines for contributing to the project.

## Code of Conduct

This project adheres to a code of conduct (see [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)). By participating, you are expected to uphold this code.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/yourusername/ola-core.git`
3. Create a branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Test your changes (see Testing below)
6. Commit with clear messages
7. Push and create a Pull Request

## Development Setup

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build project
cd ola/core
cargo build

# Run tests
cargo test
./check_all.sh
```

## Testing

Before submitting a PR, ensure:

1. **Build passes**: `cargo build`
2. **Tests pass**: `cargo test`
3. **Integration tests pass**: `./check_all.sh`
4. **No clippy warnings**: `cargo clippy -- -D warnings` (if available)
5. **Code is formatted**: `cargo fmt` (if available)

## Pull Request Guidelines

### PR Title Format

Use conventional commits style:
- `feat: add new camera detection method`
- `fix: correct socket permission issue`
- `docs: update installation guide`
- `chore: update dependencies`
- `refactor: simplify error handling`
- `test: add shutdown integration tests`

### PR Description

Include:
- **What**: Brief description of changes
- **Why**: Problem being solved or feature being added
- **How**: Approach taken (if non-obvious)
- **Testing**: How you tested the changes
- **Related Issues**: Link to related issues (e.g., "Fixes #123")

### Code Review Process

1. Maintainers will review within 3-5 business days
2. Address feedback by pushing new commits
3. Once approved, maintainers will merge

## Coding Standards

### Rust Code

- Follow Rust naming conventions
- Add doc comments for public APIs
- Use `anyhow::Result` for error handling
- Prefer `async` for I/O operations
- Add unit tests for new functions

### Commit Messages

- Use present tense ("add feature" not "added feature")
- Limit first line to 72 characters
- Reference issues and PRs where appropriate

## Areas for Contribution

### Good First Issues

- Documentation improvements
- Adding unit tests
- Improving error messages
- Code cleanup and refactoring

### Sprint 2 Priorities

- Graceful shutdown drain window
- Metrics and observability
- Automated shutdown tests
- Fuzz testing

### Future Sprints

- Face detection integration (ONNX)
- Encrypted database implementation
- PAM module integration

## Questions?

- Open a [Discussion](https://github.com/yourusername/ola-core/discussions)
- Check existing [Issues](https://github.com/yourusername/ola-core/issues)
- Read the [README](README.md)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
