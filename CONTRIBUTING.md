# Contributing to ZeroEntropy Rust SDK

Thank you for your interest in contributing to the ZeroEntropy Rust SDK! We welcome contributions from the community.

## Getting Started

1. Fork the repository on GitHub
2. Clone your fork locally
3. Create a new branch for your feature or bugfix
4. Make your changes
5. Run tests to ensure everything works
6. Submit a pull request

## Development Setup

```bash
# Clone the repository
git clone https://github.com/davidatoms/zeroentropy-rust.git
cd zeroentropy-rust

# Build the project
cargo build

# Run tests
cargo test

# Run examples (requires API key)
export ZEROENTROPY_API_KEY="your-api-key"
cargo run --example basic
```

## Code Style

- Follow standard Rust conventions and idioms
- Use `cargo fmt` to format your code
- Run `cargo clippy` to catch common mistakes
- Write clear, descriptive commit messages

## Testing

- Add tests for new functionality
- Ensure all existing tests pass
- Test your changes with real API calls when possible

## Pull Request Process

1. Update the README.md with details of changes if applicable
2. Update the CHANGELOG.md with your changes
3. Ensure your code builds and all tests pass
4. The PR will be reviewed by maintainers
5. Once approved, your changes will be merged

## Reporting Issues

When reporting issues, please include:

- A clear, descriptive title
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- Your environment (OS, Rust version, SDK version)
- Any relevant error messages or logs

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers and help them learn
- Focus on constructive feedback
- Maintain professionalism

## Questions?

If you have questions about contributing, feel free to:

- Open an issue for discussion
- Contact the maintainers at founders@zeroentropy.dev

## License

By contributing, you agree that your contributions will be licensed under the Apache 2.0 License.
