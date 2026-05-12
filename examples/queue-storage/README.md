# queue-storage example

Demonstrates the queue-driven `message/send` flow with selectable
`Storage` backends — in-memory (default) or Redis. Mirrors the Go ADK's
async task lifecycle: `message/send` enqueues and returns immediately;
a configurable number of background workers drain the queue and route
each task to the active or dead-letter store based on its terminal
state.

The server is deliberately *not* an LLM agent. A `SleepEchoHandler`
sleeps for a configurable delay (`EXAMPLE_DELAY_MS`, default 2000) and
returns `echo: <input>`. The slow handler makes worker concurrency
*visible* in the client log.

## What's in the box

```
queue-storage/
├── server/main.rs              SleepEchoHandler + env-driven storage
├── server/.well-known/agent.json
├── client/main.rs              dispatches N tasks, polls all to terminal, prints timestamps
├── docker-compose.yaml         single file driven by profiles
└── .env.redis                  flips provider=redis + workers=4 in one shot
```

## Running

### In-memory (default, 1 worker)

```sh
docker compose up --build
```

Expected client log (5 tasks × 2s delay, serial):

```
[0.00s] enqueued task #0 → state=TaskStateSubmitted
[0.01s] enqueued task #1 → state=TaskStateSubmitted
[0.01s] enqueued task #2 → state=TaskStateSubmitted
[0.02s] enqueued task #3 → state=TaskStateSubmitted
[0.02s] enqueued task #4 → state=TaskStateSubmitted
[0.02s] polling all 5 tasks to terminal state …
[2.10s] task #0 → state=TaskStateCompleted reply="echo: hello #0"
[4.20s] task #1 → state=TaskStateCompleted reply="echo: hello #1"
[6.30s] task #2 → state=TaskStateCompleted reply="echo: hello #2"
[8.40s] task #3 → state=TaskStateCompleted reply="echo: hello #3"
[10.50s] task #4 → state=TaskStateCompleted reply="echo: hello #4"
done — total wall time 10.50s for 5 tasks
```

### Redis (4 workers)

```sh
docker compose --env-file .env.redis --profile redis up --build
```

The `--profile redis` flag pulls in a `redis:8-alpine` sidecar; the
`.env.redis` file sets `A2A_QUEUE_PROVIDER=redis`,
`A2A_QUEUE_URL=redis://redis:6379`, `A2A_QUEUE_WORKERS=4`.

Expected client log (5 tasks × 2s delay, 4 workers → 2 batches):

```
[0.00s] enqueued task #0 …
…
[0.02s] polling all 5 tasks to terminal state …
[2.10s] task #0 → state=TaskStateCompleted
[2.11s] task #1 → state=TaskStateCompleted
[2.11s] task #2 → state=TaskStateCompleted
[2.12s] task #3 → state=TaskStateCompleted
[4.20s] task #4 → state=TaskStateCompleted
done — total wall time 4.20s for 5 tasks
```

The 4 workers each grab one task, sleep for 2s, complete. The 5th task
waits for a worker to free up — its completion lands at ~4s.

## Tuning

All env vars are overridable per `docker compose up` invocation or via
your own env file:

| Var | Default (memory) | Default (.env.redis) | Description |
|---|---|---|---|
| `A2A_QUEUE_PROVIDER` | `memory` | `redis` | Storage backend selector |
| `A2A_QUEUE_URL` | (unset) | `redis://redis:6379` | Redis connection URL |
| `A2A_QUEUE_NAMESPACE` | `a2a` | `a2a` | Key prefix for Redis backends |
| `A2A_QUEUE_WORKERS` | `1` | `4` | Concurrent workers draining the queue |
| `EXAMPLE_DELAY_MS` | `2000` | `2000` | Per-task sleep in the demo handler |
| `EXAMPLE_TASKS` | `5` | `5` | How many tasks the client dispatches |

## Restart durability (Redis only)

The queue lives in Redis, not in the server process, so a server crash
or restart doesn't lose work. Try it:

```sh
# Terminal 1
docker compose --env-file .env.redis --profile redis up --build

# Terminal 2 — while the client is mid-poll
docker compose --env-file .env.redis stop server
sleep 5
docker compose --env-file .env.redis --profile redis start server
```

The remaining tasks resume on the new server instance — the client's
poll loop catches the completion as normal. With the in-memory
backend, the same sequence would lose every queued task.

## How this connects to the library

The server uses `Config::from_env()` to populate `config.queue_config`
from the `A2A_QUEUE_*` vars, then passes it to `A2AServerBuilder`. The
builder calls `create_storage(&config.queue_config)` internally, which
picks `InMemoryStorage` or `RedisStorage` based on `provider`. No code
in the example explicitly mentions Redis; switching backends is a pure
config-time decision.

The image is built with `--features redis` so the same binary supports
both providers — `cargo build --features redis` makes `RedisStorage`
available, the runtime then picks based on the env.
