# Testing Sync Without Login

End-to-end test for the offline-first double-write: every sea-orm mutation is
mirrored into the loomabase store (outbound), and store changes received during
`sync_all` are applied back into the sea-orm read path (inbound). Everything
runs against a local server and **no account** is required.

## Why no login is needed

`POST /sync` never looks up an account. The only gates are:

- a valid HS256 JWT signed with the server's `JWT_SIGNING_KEY`, and
- a non-empty `x-device-id` header.

The tenant is just the token's `user_identifier` claim
(`server/src/loomabase/mod.rs`), so a token minted for any throwaway UUID
works. The app also has a guest mode ("Continue without account",
`app/layers/desktop/pages/auth/index.vue`), so the UI is fully usable logged
out.

> **Correctness note:** app and the device-B injector must share the **same
> tenant**. `just dev-token` mints a random `user_identifier` by default, so pin
> one with `--user-id` and reuse it for both sides.

## Prerequisites

- `just`, Docker, Node, Rust toolchains installed
- Server env ready (see below)

Pick a tenant before anything else:

```bash
export SYNC_USER="$(uuidgen | tr 'A-Z' 'a-z')"
```

## 1. Start the server

The server requires these env vars (`server/src/config/env.rs`):

```
PORT=8000
DATABASE_URL=postgres://orchard:orchard@localhost:6543/orchard
MAX_DB_CONNECTIONS=4
BODY_LIMIT_MB=25
ENVIRONMENT=development
JWT_SIGNING_KEY=<any string; keep it stable>
```

**Option A — Docker stack (matches `just watch-server`):**

Put the vars above (plus `DATABASE_USER`, `DATABASE_PASSWORD`, `DATABASE_NAME`)
in the root `.env`, then:

```bash
just watch-server
```

Wait for logs to show migrations + sync schema init (boot creates all 8
`loomabase_*` / `*_crdt` tables).

**Option B — Cargo (uses `server/.env`):**

```bash
docker compose up -d postgres      # exposes postgres on localhost:6543
# write the vars above into server/.env (the server loads it via dotenv)
cd server && cargo run
```

> The server's `JWT_SIGNING_KEY` must match the key used when minting. If the
> key lives only in root `.env`, export it before minting:
> `export JWT_SIGNING_KEY=<the server's key>`.

## 2. Mint a pinned token

```bash
export SYNC_USER="7f3c2e91-8b64-4a0d-b5e7-1c9f6a23d814"
export LUNAR_DEV_TOKEN="$(just dev-token -- --user-id "$SYNC_USER")"
echo "$LUNAR_DEV_TOKEN" | cut -d. -f2 | base64 -d   # sanity: decode = your SYNC_USER
```

`just dev-token` (`scripts/mint-dev-token.sh`) signs the JWT with
`JWT_SIGNING_KEY` (env or `server/.env`), so it must match the running server.

## 3. Launch the app as guest

```bash
cd console
npm run tauri dev
```

The app inherits `LUNAR_DEV_TOKEN`; `LOOMABASE_SYNC_URL` defaults to
`http://localhost:8000/sync`. At the auth screen click **"Continue without
account"**, then use the app normally.

## 4. Outbound check

Create a todo (e.g. title `sync out`) and click the sync button in the titlebar.
The double-write puts its cells into the Postgres CRDT table:

```bash
psql postgres://orchard:orchard@localhost:6543/orchard \
  -c "SELECT todo_id, column_name, value FROM todos_crdt WHERE tenant_id='$SYNC_USER' ORDER BY seq DESC LIMIT 10;"
```

You should see a `title` cell with value `"sync out"` and the app's device id.

## 5. Inbound check (device B over HTTP)

`dev_device_b` opens an in-memory loomabase store, creates (or re-titles) a
todo, and pushes it through the real `/sync` endpoint — no second Tauri
instance needed:

```bash
just dev-device-b sync-b-1 "hello from B"
```

Then click sync in the app → the "hello from B" todo appears in the todo list,
applied through `mirror::apply_all` into the sea-orm read path.

## One-shot alternative

The app must already be running with the pinned token for convergence.

```bash
# from the app: launch with LUNAR_DEV_TOKEN pinned to $SYNC_USER as above,
# then on the CLI:
SYNC_USER=<same uuid> LUNAR_DEV_TOKEN="$(just dev-token -- --user-id "$SYNC_USER")" \
  just dev-device-b sync-b-2 "from device B"
```

Note: `just dev-sync` mints a **fresh random token**, which creates a different
tenant — it will not converge with the app. Prefer the pinned-token +
`just dev-device-b` path above.

## Troubleshooting

- **401 / invalid token** — the mint key and the server's `JWT_SIGNING_KEY` differ.
- **`SyncPageBudgetExhausted` overnight** — server down, old token, or wrong tenant.
- **Nothing appears in the app after device B** — ensure the app's sync ran
  while online, and both tokens decode to the *same* `user_identifier`.
- **Cross-device convergence at the CRDT level** is covered separately by the
  server test: `cargo test --test offline_roundtrip` (needs this Postgres).