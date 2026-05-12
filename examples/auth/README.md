# Auth-Gated A2A Example

Demonstrates how the Rust ADK enforces bearer-token authentication on
`POST /a2a` while keeping `GET /health` and `GET /.well-known/agent.json`
public, and how `agent/getAuthenticatedExtendedCard` returns the extended
agent card only to authenticated callers.

## What this shows

- `A2AServerBuilder::with_auth_verifier(...)` wires a custom
  [`AuthVerifier`] into the server. The example uses a static-token
  verifier for the zero-deps quick demo and falls back to the bundled
  `OidcJwtVerifier` (OIDC discovery + JWKS validation) when
  `AUTH_ENABLE=true` is set.
- `POST /a2a` returns **HTTP 401** when the `Authorization` header is
  missing or malformed.
- `GET /health` and `GET /.well-known/agent.json` remain reachable
  without a token.
- `agent/getAuthenticatedExtendedCard` returns the agent card to
  callers whose bearer token verifies successfully.

## Running locally with the static-token verifier (no Docker)

Zero external dependencies. The server accepts a hard-coded token, the
client uses the same token.

In one terminal:

```bash
cargo run --example auth-server
# Or with a custom token:
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

## Running with Docker Compose (Keycloak OIDC)

`docker-compose.yaml` brings up three services on a private bridge
network:

1. **Keycloak 26.6.1** — pre-imports `keycloak/realm-export.json` on
   start, so the realm, client, and audience mapper are ready before
   any other service launches.
2. **`auth-server`** — runs with `AUTH_ENABLE=true` so the
   `A2AServerBuilder` instantiates `OidcJwtVerifier` from
   `Config::from_env()` and validates incoming JWTs against the
   Keycloak realm's JWKS.
3. **`auth-client`** — runs with `AUTH_MODE=oidc` so it first exchanges
   the configured client credentials for a real JWT at Keycloak's
   token endpoint, then calls the protected endpoint with that JWT.

```bash
cd examples/auth
docker compose up --build
```

Expected output (abbreviated):

```
keycloak  | Imported realm inference-gateway-realm
server    | enabling bearer-token auth on POST /a2a (issuer=http://keycloak:8080/realms/inference-gateway-realm)
client    | → fetching client_credentials JWT from http://keycloak:8080/...
client    | ← received access_token (1024 chars)
client    | → calling agent/getAuthenticatedExtendedCard WITHOUT a token
client    | ← server replied 401 Unauthorized (expected 401)
client    | → calling agent/getAuthenticatedExtendedCard WITH bearer token
client    | ← server replied 200 OK
client    | extended agent card: name=Auth-Gated Rust A2A Agent version=0.1.0
```

### Inspecting the realm

The Keycloak admin console is exposed on the host at
<http://localhost:8080> (login: `admin` / `admin`).

The realm export at [`keycloak/realm-export.json`](./keycloak/realm-export.json)
defines:

- realm `inference-gateway-realm`
- client `inference-gateway-client` (confidential, service accounts
  enabled) with secret `inference-gateway-client-secret`
- an `oidc-audience-mapper` that adds `inference-gateway-client` to the
  access token's `aud` claim — required because the server is
  configured with `AUTH_CLIENT_ID=inference-gateway-client` and
  `OidcJwtVerifier` validates `aud` against that value.

### Grabbing a token by hand

```bash
TOKEN=$(curl -s -X POST \
  http://localhost:8080/realms/inference-gateway-realm/protocol/openid-connect/token \
  -d 'grant_type=client_credentials' \
  -d 'client_id=inference-gateway-client' \
  -d 'client_secret=inference-gateway-client-secret' \
  | jq -r .access_token)

echo "$TOKEN" | cut -d. -f2 | base64 -d 2>/dev/null | jq .
```

Then call the server (note: the compose-bundled server only listens on
the `a2a-network` bridge, so you would need to publish its port or
exec into the container to reach it from the host).

## Switching auth backends in your own code

`A2AServerBuilder` picks an `AuthVerifier` in this order:

1. Explicit `.with_auth_verifier(verifier)` if you supply one (this
   wins regardless of `AuthConfig`).
2. Otherwise, when `.with_config(cfg)` is called and
   `cfg.auth_config.enable == true`, the builder instantiates
   `OidcJwtVerifier::from_config(...)` automatically.
3. Otherwise no auth middleware is attached and all routes are public.

The example server picks between (1) and (2) at startup based on the
`AUTH_ENABLE` env var.
