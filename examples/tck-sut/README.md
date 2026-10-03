# A2A TCK System Under Test

An ADK server that implements the [A2A TCK](https://github.com/a2aproject/a2a-tck) scenarios
(`scenarios/core_operations.feature` and `scenarios/streaming.feature`). The TCK selects each
scenario through the `messageId` prefix it sends (`tck-complete-task`, `tck-artifact-text`, ...).
CI runs the TCK JSON-RPC suite (MUST, SHOULD and MAY) against it on every pull request.

## Running the TCK locally

```bash
cargo run -p tck-sut
```

In another terminal:

```bash
git clone https://github.com/a2aproject/a2a-tck && cd a2a-tck
uv run ./run_tck.py --sut-host http://localhost:9999 --transport jsonrpc
```

## Deselected in CI

None: the gaps those `--deselect` flags covered (blocking `SendMessage`, direct `Message`
replies, push notification delivery, agent card caching headers) are implemented, so the
three `--deselect` lines in `.github/workflows/ci.yml` can go.

## Compatibility score

The TCK's overall percentage stays below 100% even when no test fails. Since a2a-tck #218,
requirements that never record a result count as failures (`NOT TESTED`), and at the pinned TCK
commit 25 requirements have no TCK test at all (`AUTH-*`, `BIND-EQUIV-*`, `CARD-SIGN-*`,
`VER-CLIENT-*`, `VER-SERVER-001`, `GRPC-SVC-003`), while `HTTP_JSON-SVC-001` skips without
recording a result when HTTP+JSON is not served. Judge a change by the pytest summary instead.

## Not covered

- gRPC and HTTP+JSON transports: the ADK serves JSON-RPC only.
- Required extensions (`CORE-CAP-004`): the ADK does not enforce `required: true` extensions.
- Tests for a capability this agent declares (streaming, push notifications, extended card) when it
  is absent; unit tests cover those error paths.
