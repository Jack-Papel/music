# Contributing to Symphoxy

Thank you for your interest in contributing to Symphoxy! This document provides guidelines and information for contributors.

## Getting Started

1. Fork the repository on GitHub
2. Clone your fork locally
3. Create a feature branch from `main`
4. Make your changes
5. Test your changes
6. Submit a pull request

## Development Setup

### Prerequisites

- Rust 1.73+ (latest stable recommended)
- System audio libraries:
  - Linux: `libasound2-dev` (Ubuntu/Debian) or `alsa-lib-devel` (RHEL/CentOS)
  - macOS: No additional dependencies
  - Windows: No additional dependencies

### Building

```bash
# Clone your fork
git clone https://github.com/Jack-Papel/symphoxy.git
cd symphoxy

# Build the project
cargo build

# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run lints
cargo clippy -- -D warnings
```

## Code Style

- VSCode is recommended, as there are settings for vscode included in the repository.
- If you don't use vscode:
  - Use `cargo fmt` to format your code
  - Run `cargo clippy` and fix any warnings, or suppress them if you have a good reason
- Follow Rust naming conventions
- Add documentation for public APIs
- Include examples in documentation where helpful

## Testing

- Add tests for complex functionality
- Ensure all existing tests pass
- Include doc tests for public APIs
- Test with different feature combinations, if applicable

## Documentation

- Document all public APIs with doc comments
- Include examples in documentation
- Update README.md if adding major features
- Update CHANGELOG.md following [Keep a Changelog](https://keepachangelog.com/) format

## Pull Request Process

1. Ensure your branch is up to date with main
2. Run the full test suite
3. Update documentation as needed
4. Add entries to CHANGELOG.md
5. Submit your pull request with:
   - Clear description of changes
   - Reference to any related issues
   - Screenshots/audio samples if applicable

## Issue Guidelines

When creating issues:

- Use clear, descriptive titles
- Provide steps to reproduce bugs
- Include relevant system information
- For feature requests, explain the use case

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers and help them learn
- Focus on constructive feedback
- Celebrate diverse perspectives

## Musical Content

When contributing examples or test music:

- Ensure you have rights to any musical content
- Prefer simple, original melodies for examples
- Consider using public domain music for larger examples

## Questions?

Feel free to open an issue for questions about contributing, or reach out to the maintainers.

Thank you for contributing to Symphoxy! 🎵
