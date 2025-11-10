# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2025-11-10

### Changed
- Updated repository URL in Cargo.toml
- Improved documentation and examples

## [0.1.0] - 2025-11-08

### Added
- Initial release of ZeroEntropy Rust SDK
- Complete implementation of ZeroEntropy API endpoints
- Collections API (create, list, delete)
- Documents API (add text/PDF, get info, update, delete)
- Queries API (top documents, top snippets, top pages)
- Reranking API
- Async client built on Tokio and reqwest
- Configurable retry logic with exponential backoff
- Strong typing with comprehensive error handling
- Builder pattern for client configuration
- Support for metadata filtering
- PDF document support
- Examples: basic usage and arXiv search
- Complete API documentation
- Apache 2.0 license

[0.1.1]: https://github.com/davidatoms/zeroentropy-rust/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/davidatoms/zeroentropy-rust/releases/tag/v0.1.0
