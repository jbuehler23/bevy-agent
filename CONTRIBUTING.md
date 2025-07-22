# Contributing to Bevy AI Agent

Thank you for your interest in contributing to Bevy AI Agent! This document provides guidelines and information for contributors.

## Code of Conduct

This project adheres to a [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## How to Contribute

### Reporting Issues

1. **Check existing issues** first to avoid duplicates
2. **Use the issue templates** when creating new issues
3. **Provide clear reproduction steps** for bugs
4. **Include relevant environment information**

### Contributing Code

1. **Fork the repository**
2. **Create a feature branch** from `main`
3. **Make your changes** following our coding standards
4. **Add tests** for new functionality
5. **Update documentation** as needed
6. **Submit a pull request**

### Pull Request Process

1. Ensure your PR passes all CI checks
2. Update the README.md if needed
3. Update documentation for new features
4. Add appropriate tests
5. Link any related issues

## Development Setup

### Prerequisites

- Rust 1.70.0 or later
- Git

### Local Development

```bash
# Clone the repository
git clone https://github.com/jbuehler23/bevy-agent.git
cd bevy-agent

# Install dependencies and build
cargo build

# Run tests
cargo test

# Run examples
cargo run --example basic_generation

# Run the CLI
cargo run -- --help
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with all features
cargo test --all-features

# Run documentation tests
cargo test --doc

# Run examples
cargo check --examples
```

### Code Quality

We use several tools to maintain code quality:

```bash
# Format code
cargo fmt

# Run clippy lints
cargo clippy -- -D warnings

# Check documentation
cargo doc --all-features --no-deps
```

## Coding Standards

### Rust Style

- Follow standard Rust formatting (`cargo fmt`)
- Use `clippy` suggestions (`cargo clippy`)
- Write comprehensive documentation
- Add `#[cfg(test)]` for test modules

### Documentation

- All public APIs must be documented
- Include examples in documentation
- Use doc tests where appropriate
- Update README.md for user-facing changes

### Testing

- Write unit tests for new functionality
- Add integration tests for complex features
- Ensure examples compile and run
- Test error conditions

### Git Commit Messages

Use conventional commit format:

```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: new feature
- `fix`: bug fix
- `docs`: documentation
- `style`: formatting changes
- `refactor`: code refactoring
- `test`: adding tests
- `chore`: maintenance

Examples:
- `feat(ai): add Claude-3 support`
- `fix(cli): resolve config file parsing issue`
- `docs(readme): update installation instructions`

## Architecture

### Project Structure

```
bevy-agent/
├── src/
│   ├── ai/          # AI provider implementations
│   ├── cli/         # Command-line interface
│   ├── config/      # Configuration management
│   ├── game_templates/ # Built-in game templates
│   ├── project/     # Project management
│   ├── utils/       # Utility functions
│   ├── error.rs     # Error types
│   ├── lib.rs       # Library entry point
│   └── main.rs      # CLI entry point
├── examples/        # Usage examples
├── tests/          # Integration tests
└── docs/           # Additional documentation
```

### Adding New AI Providers

1. Add configuration struct in `src/config.rs`
2. Implement provider in `src/ai/` module
3. Add model types to `ModelType` enum
4. Update agent logic in `src/ai/agent.rs`
5. Add tests and documentation

### Adding New Commands

1. Add command struct in `src/cli.rs`
2. Implement command handler
3. Add subcommand to main CLI enum
4. Add tests and documentation
5. Update help text

## Release Process

Releases are automated through GitHub Actions:

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create a git tag: `git tag v0.1.1`
4. Push tag: `git push origin v0.1.1`
5. GitHub Actions will build and publish to crates.io

## Getting Help

- Open an issue for bugs or feature requests
- Start a discussion for questions
- Check existing documentation and examples

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (MIT OR Apache-2.0).
