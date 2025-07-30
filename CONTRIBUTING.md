# Contributing to Rust ADK

Thank you for your interest in contributing to the Rust Agent Development Kit (ADK)! This guide will help you get started with contributing to this project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Pull Request Process](#pull-request-process)
- [Issue Reporting](#issue-reporting)
- [Project Structure](#project-structure)
- [Available Tasks](#available-tasks)

## Code of Conduct

By participating in this project, you agree to abide by our Code of Conduct. Please be respectful and constructive in all interactions.

## Getting Started

### Prerequisites

- **Rust**: Version 1.88 or later
- **Task**: Task runner for development workflows
- **Git**: For version control
- **Docker**: For running development containers (optional)

### Setting Up Your Development Environment

1. **Fork and Clone the Repository**

   ```bash
   git clone https://github.com/YOUR_USERNAME/rust-adk.git
   cd rust-adk
   ```

2. **Install Dependencies**

   ```bash
   cargo build
   ```

3. **Verify Your Setup**
   ```bash
   task test
   task lint
   task analyse
   ```

## Development Workflow

We follow a structured development workflow to maintain code quality:

### Before Making Changes

1. Create a new branch from `main`:

   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes following our [coding standards](#coding-standards)

### Before Committing

**Always run these commands before committing:**

```bash
# 1. Run linting to ensure code formatting
task lint

# 2. Run static analysis to catch potential issues
task analyse

# 3. Run tests to ensure all functionality works
task test
```

If any of these fail, fix the issues before proceeding.

### Configuration Changes

When adding new configuration fields:

1. Run `task lint` to ensure code quality
2. Run `task analyse` to catch potential issues
3. Run `task test` to ensure all tests pass
4. Update the README.md file or any documentation files with the recently added implementation

## Coding Standards

### General Principles

- **Type Safety**: Always prefer type safety over dynamic typing. Use strong typing and interfaces to ensure type safety and reduce runtime errors.
- **Simplicity First**: Always search for the simplest solution first before considering more complex alternatives.
- **Interface-Driven Design**: When possible, code to an interface so it's easier to mock in tests.
- **Early Returns**: Favor early returns to simplify logic and avoid deep nesting with if-else structures.
- **Switch Over If-Else**: Prefer switch statements over if-else chains for cleaner and more readable code when checking multiple conditions.

### Code Style

- Follow Rust's standard formatting conventions
- Use `cargo fmt` for consistent formatting
- Write clear, self-documenting code with meaningful variable and function names
- Add comprehensive documentation for public APIs

### Error Handling

- Use `Result<T, E>` for recoverable errors
- Use `anyhow` for error context and `thiserror` for custom error types
- Provide meaningful error messages that help users understand what went wrong

## Testing Guidelines

### Test Philosophy

- **Table-Driven Testing**: Always prefer table-driven testing when writing tests
- **Isolated Tests**: Each test case should have its own isolated mock server and mock dependencies for easier understanding and maintenance
- **Comprehensive Coverage**: Aim for high test coverage, especially for public APIs

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_feature_name() {
        // Arrange: Set up test data and mocks
        let test_cases = vec![
            TestCase {
                name: "valid_input",
                input: "test_input",
                expected: Ok("expected_output"),
            },
            TestCase {
                name: "invalid_input",
                input: "invalid",
                expected: Err("expected_error"),
            },
        ];

        for case in test_cases {
            // Act: Execute the function under test
            let result = function_under_test(case.input).await;

            // Assert: Verify the result
            assert_eq!(result, case.expected, "Test case: {}", case.name);
        }
    }
}
```

### Mock Strategy

- Create isolated mock servers for each test case
- Use dependency injection to make components testable
- Mock external dependencies to ensure tests are deterministic

## Pull Request Process

### Before Submitting

1. **Update Documentation**: Ensure all documentation is updated to reflect your changes
2. **Add Tests**: Include comprehensive tests for your changes
3. **Run Quality Checks**: Ensure all tasks pass:
   ```bash
   task lint && task analyse && task test
   ```

### PR Guidelines

1. **Clear Description**: Provide a clear description of what your PR does
2. **Link Issues**: Reference any related issues
3. **Breaking Changes**: Clearly mark any breaking changes
4. **Examples**: Include usage examples if adding new features

### Code Review Process

- PRs require review from maintainers
- Address all feedback before merge
- Use `@claude /review` in PR comments to trigger automated code review
- Automated reviews will check for:
  - Code quality and best practices
  - Potential bugs or issues
  - Performance considerations
  - Security concerns
  - Test coverage

## Issue Reporting

### Bug Reports

When reporting bugs, please include:

- **Environment**: Rust version, OS, and ADK version
- **Steps to Reproduce**: Clear, numbered steps
- **Expected Behavior**: What you expected to happen
- **Actual Behavior**: What actually happened
- **Code Sample**: Minimal reproducing example
- **Error Messages**: Complete error messages and stack traces

### Feature Requests

For feature requests, please provide:

- **Use Case**: Why this feature would be useful
- **Proposed API**: How you envision the API working
- **Alternatives**: Alternative solutions you've considered
- **Implementation Ideas**: Any thoughts on implementation

## Project Structure

```
rust-adk/
├── src/
│   ├── a2a_types.rs     # Generated A2A protocol types
│   ├── client.rs        # A2A client implementation
│   ├── config.rs        # Configuration management
│   ├── lib.rs           # Library entry point
│   └── server.rs        # A2A server implementation
├── examples/            # Usage examples
├── tests/               # Integration tests
├── Taskfile.yml         # Development tasks
└── schema.{json,yaml}   # A2A protocol schema
```

## Available Tasks

Our project uses [Task](https://taskfile.dev/) for development workflows:

### Core Development Tasks

- `task lint` - Run code formatting checks
- `task lint:fix` - Fix formatting issues automatically
- `task analyse` - Run static analysis with Clippy
- `task test` - Run all tests

### Schema Management

- `task a2a:download-schema` - Download latest A2A schema
- `task a2a:generate-types` - Generate Rust types from schema

### Examples

- `task examples:minimal-server` - Run minimal server example
- `task examples:configured-server` - Run configured server example
- `task examples:server-with-toolbox` - Run server with tools example
- `task examples:client` - Run client example

## Tools and MCPs

This project leverages several tools for enhanced development:

- **context7**: Helps find the latest updates, features, or best practices of libraries relevant to tasks
- **Claude Code Review**: Automated code review triggered by `@claude /review` in PR comments

## Related Repositories

This ADK is part of the larger Inference Gateway ecosystem:

- [Inference Gateway](https://github.com/inference-gateway)
- [Inference Gateway UI](https://github.com/inference-gateway/ui)
- [Go ADK](https://github.com/inference-gateway/adk)
- [TypeScript ADK](https://github.com/inference-gateway/typescript-adk)
- [Go SDK](https://github.com/inference-gateway/sdk)
- [TypeScript SDK](https://github.com/inference-gateway/typescript-sdk)
- [Python SDK](https://github.com/inference-gateway/python-sdk)
- [Rust SDK](https://github.com/inference-gateway/rust-sdk)
- [Kubernetes Operator](https://github.com/inference-gateway/operator)
- [Agent Definition Language](https://github.com/inference-gateway/adl)
- [Documentation](https://github.com/inference-gateway/docs)

## Getting Help

- **Documentation**: Check the [API documentation](https://docs.rs/inference-gateway-adk)
- **Examples**: Look at the `examples/` directory
- **Issues**: Search existing issues or create a new one
- **Discussions**: Use GitHub Discussions for questions and ideas

## License

By contributing to this project, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to the Rust ADK! Your efforts help make AI agent development more accessible and powerful for everyone.
