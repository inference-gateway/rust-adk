# Auth-Gated A2A Example

Demonstrates how the Rust ADK enforces bearer-token authentication on
`POST /a2a` while keeping `GET /health` and `GET /.well-known/agent.json`
public, and how `agent/getAuthenticatedExtendedCard` returns the extended
agent card only to authenticated callers.

## What this shows

- `A2AServerBuilder::with_auth_verifier(...)` wires a custom
  [`AuthVerifier`] into the server (the example uses a static-token
  verifier so the demo runs without an OIDC issuer).
- `POST /a2a` returns **HTTP 401** when the `Authorization` header is
  missing or malformed.
- `GET /health` and `GET /.well-known/agent.json` remain reachable
  without a token.
- `agent/getAuthenticatedExtendedCard` returns the agent card to
  callers whose bearer token verifies successfully.

## Switching to a real OIDC issuer

Replace the static-token verifier with the bundled `OidcJwtVerifier`
(default when `AUTH_ENABLE=true`):

```bash
AUTH_ENABLE=true \
AUTH_ISSUER_URL=https://keycloak.example/realms/inference-gateway-realm \
AUTH_CLIENT_ID=inference-gateway-client \
cargo run --example auth-server
```

When `AUTH_ENABLE=true`, `Config::from_env()` instantiates
`OidcJwtVerifier` automatically, performs OIDC discovery
(`<issuer>/.well-known/openid-configuration`), pulls the JWKS, and
validates incoming JWTs against the issuer, audience (`client_id`), and
signature.

## Running locally

In one terminal:

```bash
cargo run --example auth-server
# or set a custom token:
# EXAMPLE_BEARER_TOKEN=my-secret cargo run --example auth-server
```

In another terminal:

```bash
cargo run --example auth-client
# Token used by the client must match the server:
# EXAMPLE_BEARER_TOKEN=my-secret cargo run --example auth-client
```

You can also poke the endpoints directly with curl:

```bash
# Public - works without a token
curl http://localhost:8081/health
curl http://localhost:8081/.well-known/agent.json

# Protected - 401 without a token
curl -i http://localhost:8081/a2a \
  -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","id":"1","method":"agent/getAuthenticatedExtendedCard","params":{"tenant":"demo-tenant"}}'

# Protected - 200 with a valid token
curl http://localhost:8081/a2a \
  -H 'Authorization: Bearer demo-token-123' \
  -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","id":"1","method":"agent/getAuthenticatedExtendedCard","params":{"tenant":"demo-tenant"}}'
```
