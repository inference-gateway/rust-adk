# input-required example

Demonstrates the `TaskStateInputRequired` state. A non-LLM
`WeatherHandler` inspects each incoming message and decides:

- if it mentions a known city → set state to `TaskStateCompleted` with
  a hardcoded weather reply,
- otherwise → set state to `TaskStateInputRequired` with a follow-up
  question asking which city.

The client dispatches **two independent tasks**, one of each shape, so
the two branches are visible side by side.

## What's in the box

```
input-required/
├── server/main.rs                 WeatherHandler with branching state
├── server/.well-known/agent.json  Agent metadata loaded at startup
├── client/main.rs                 Two dispatches: with-city and no-city
├── docker-compose.yaml            Server + client only (no inference-gateway)
└── README.md
```

## What this shows

- **`TaskStateInputRequired`** is a non-terminal state. The server
  keeps tasks in this state pending in the active store (they don't
  move to dead-letter), which is the correct shape for "waiting on
  the user for more info".
- A custom `TaskHandler` can branch on message content and choose the
  state without any LLM involvement.

Expected client log:

```
→ [with-city] sending: What's the weather in London right now?
  [with-city] task <id> accepted in state TaskStateSubmitted
  [with-city] task <id> settled in state TaskStateCompleted
  [with-city] agent says: Weather in london: 18°C, partly cloudy.
→ [no-city] sending: What's the weather?
  [no-city] task <id> accepted in state TaskStateSubmitted
  [no-city] task <id> settled in state TaskStateInputRequired
  [no-city] agent says: Which city would you like the weather for? …
```

## Note on the resume path

In the current rust-adk, `message/send` always creates a new task,
even when the request carries an existing `task_id`. That means the
"second send resumes the paused task" flow demonstrated by some other
A2A implementations is not yet wired here — the `TaskStateInputRequired`
state surfaces, but the protocol-level resume is a separate library
improvement. This example sticks to demonstrating the state itself
until that path lands.

## Running with Docker Compose

```bash
cd examples/input-required
docker compose up --build
```

No `.env` needed — there are no provider keys.

## Running locally

```bash
cd examples/input-required/server
cargo run --example input-required-server
# or: task examples:input-required-server

cargo run --example input-required-client
# or: task examples:input-required-client
```

The server listens on `0.0.0.0:8087`. The client honours `SERVER_URL`
(default `http://localhost:8087`).
