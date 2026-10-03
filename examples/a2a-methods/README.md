# `a2a-methods/`

One runnable client example per JSON-RPC method in the A2A specification,
sharing a single offline server. Every example uses the typed request /
response structs from [`inference_gateway_adk::a2a_types`] - no hand-rolled
JSON envelopes.

## Layout

```text
a2a-methods/
├── docker-compose.yaml                          # Server + one Compose profile per client
├── server/main.rs                               # shared offline server (echo fallback)
└── client/
    ├── message_send.rs                          # SendMessage
    ├── message_stream.rs                        # SendStreamingMessage
    ├── tasks_get.rs                             # GetTask
    ├── tasks_list.rs                            # ListTasks
    ├── tasks_cancel.rs                          # CancelTask
    ├── tasks_resubscribe.rs                     # SubscribeToTask
    ├── push_config_set.rs                       # CreateTaskPushNotificationConfig
    ├── push_config_get.rs                       # GetTaskPushNotificationConfig
    ├── push_config_list.rs                      # ListTaskPushNotificationConfigs
    ├── push_config_delete.rs                    # DeleteTaskPushNotificationConfig
    └── agent_authenticated_extended_card.rs     # GetExtendedAgentCard
```

## Running with Docker Compose

The compose manifest builds one long-running `server` container plus eleven
per-method client containers, each parked behind its own
[Compose profile][compose-profiles] so a bare `docker compose up` doesn't
fan out into eleven parallel runs.

```bash
cd examples/a2a-methods

# Pick a single method to exercise:
docker compose --profile message-send                       up --build
docker compose --profile message-stream                     up --build
docker compose --profile tasks-get                          up --build
docker compose --profile tasks-list                         up --build
docker compose --profile tasks-cancel                       up --build
docker compose --profile tasks-resubscribe                  up --build
docker compose --profile push-config-set                    up --build
docker compose --profile push-config-get                    up --build
docker compose --profile push-config-list                   up --build
docker compose --profile push-config-delete                 up --build
docker compose --profile agent-authenticated-extended-card  up --build

# Or run every client in sequence against the same server:
docker compose --profile all-clients up --build
```

The selected profile pulls the `server` service in as a dependency, so you
never need to start it by hand. No `.env` file is required - the server is
offline (no Inference Gateway, no LLM credentials).

[compose-profiles]: https://docs.docker.com/compose/profiles/

## Running locally

In one terminal, start the server:

```bash
cargo run -p a2a-methods-server
```

In another terminal, run any of the per-method clients:

```bash
cargo run -p a2a-methods-client --bin message-send
cargo run -p a2a-methods-client --bin message-stream
cargo run -p a2a-methods-client --bin tasks-get
cargo run -p a2a-methods-client --bin tasks-list
cargo run -p a2a-methods-client --bin tasks-cancel
cargo run -p a2a-methods-client --bin tasks-resubscribe
cargo run -p a2a-methods-client --bin push-config-set
cargo run -p a2a-methods-client --bin push-config-get
cargo run -p a2a-methods-client --bin push-config-list
cargo run -p a2a-methods-client --bin push-config-delete
cargo run -p a2a-methods-client --bin agent-authenticated-extended-card
```

The server listens on port `8085` by default (override with `SERVER_PORT=…`).
Clients respect `SERVER_URL` and default to `http://localhost:8085`.

## Notes

- No LLM is wired up. `SendMessage` and `SendStreamingMessage` fall through to the
  built-in offline echo reply, so each client runs end-to-end without external
  credentials.
- Examples that mutate state (e.g. `CancelTask`,
  the `*TaskPushNotificationConfig(s)` methods) seed their own task via
  `SendMessage` first so they remain self-contained and re-runnable.
- Webhook *delivery* for push notifications is tracked in a separate ticket;
  the four `*TaskPushNotificationConfig(s)` methods here exercise the control plane
  (storage + retrieval) only.
- The shared example server opts into the extended agent card by setting
  `capabilities.extendedAgentCard: true` on the static agent card it advertises;
  this is what lets `a2a-methods-agent-authenticated-extended-card` succeed
  rather than receive `METHOD_NOT_FOUND`. Production agents should gate the
  flag on their own auth policy.
- `SubscribeToTask` lets a client re-attach to an existing
  task id and receive a snapshot of its current state
  followed by any remaining `TaskStatusUpdateEvent` deltas. The example
  here seeds a task via `SendMessage` (which the echo handler completes
  immediately) so the resubscribed stream emits a snapshot and a terminal
  terminal status update.
