# Custom Instructions for Copilot

- Always use context7 to check for the latest updates, features, or best practices of a library relevant to the task at hand.
- Always prefer Table-Driven Testing: When writing tests.
- Always use Early Returns: Favor early returns to simplify logic and avoid deep nesting with if-else structures.
- Always prefer switch statements over if-else chains: Use switch statements for cleaner and more readable code when checking multiple conditions.
- Always run `task analyse` to ensure code quality and catch potential issues before committing.
- Always run `task lint` before committing code to ensure it adheres to the project's linting rules.
- Always run `task test` before committing code to ensure all tests pass.
- Always search for the simplest solution first before considering more complex alternatives.
- Always prefer type safety over dynamic typing: Use strong typing and interfaces to ensure type safety and reduce runtime errors.
- When possible code to an interface so it's easier to mock in tests.
- When writing tests, each test case should have it's own isolated mock server mock dependecies so it's easier to understand and maintain.

## Development Workflow

### Configuration Changes

When adding new configuration fields:

1. Run `task lint` to ensure code quality
2. Run `task analyse` to catch potential issues
3. Run `task test` to ensure all tests pass
4. Update the README.md file or any documentation files with the recently added implementation

## Available Tools and MCPs

- context7 - Helps by finding the latest updates, features, or best practices of a library relevant to the task at hand.

## Related Repositories

- [Inference Gateway](https://github.com/inference-gateway)
  - [Inference Gateway UI](https://github.com/inference-gateway/ui)
  - [Go SDK](https://github.com/inference-gateway/go-sdk)
  - [Rust SDK](https://github.com/inference-gateway/rust-sdk)
  - [TypeScript SDK](https://github.com/inference-gateway/typescript-sdk)
  - [Documentation](https://github.com/inference-gateway/docs)

## MCP Useful links

- [Introduction](https://modelcontextprotocol.io/introduction)
- [Specification](https://modelcontextprotocol.io/specification)
- [Examples](https://modelcontextprotocol.io/examples)
- [Schema](https://raw.githubusercontent.com/modelcontextprotocol/modelcontextprotocol/refs/heads/main/schema/draft/schema.json)
