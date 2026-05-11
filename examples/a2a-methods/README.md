# `a2a-methods/`

One runnable client example per JSON-RPC method in the A2A specification,
sharing a single offline server. Every example uses the typed request /
response structs from [`inference_gateway_adk::a2a_types`] — no hand-rolled
JSON envelopes.

## Layout

```text
a2a-methods/
├── server/main.rs                       # shared offline server (echo fallback)
└── client/
    ├── message_send.rs                  # message/send
    ├── message_stream.rs                # message/stream
    ├── tasks_get.rs                     # tasks/get
    ├── tasks_list.rs                    # tasks/list
    ├── tasks_cancel.rs                  # tasks/cancel
    ├── push_config_set.rs               # tasks/pushNotificationConfig/set
    ├── push_config_get.rs               # tasks/pushNotificationConfig/get
    ├── push_config_list.rs              # tasks/pushNotificationConfig/list
    └── push_config_delete.rs            # tasks/pushNotificationConfig/delete
```

## Running

In one terminal, start the server:

```bash
cargo run --example a2a-methods-server
```

In another terminal, run any of the per-method clients:

```bash
cargo run --example a2a-methods-message-send
cargo run --example a2a-methods-message-stream
cargo run --example a2a-methods-tasks-get
cargo run --example a2a-methods-tasks-list
cargo run --example a2a-methods-tasks-cancel
cargo run --example a2a-methods-push-config-set
cargo run --example a2a-methods-push-config-get
cargo run --example a2a-methods-push-config-list
cargo run --example a2a-methods-push-config-delete
```

The server listens on port `8085` by default (override with `SERVER_PORT=…`).
Clients respect `SERVER_URL` and default to `http://localhost:8085`.

## Notes

- No LLM is wired up. `message/send` and `message/stream` fall through to the
  built-in offline echo reply, so each client runs end-to-end without external
  credentials.
- Examples that mutate state (e.g. `tasks/cancel`,
  `pushNotificationConfig/{set,get,list,delete}`) seed their own task via
  `message/send` first so they remain self-contained and re-runnable.
- Webhook *delivery* for push notifications is tracked in a separate ticket;
  the four `pushNotificationConfig/*` methods here exercise the control plane
  (storage + retrieval) only.
