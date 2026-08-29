# Round-trip: how sync works client ⇄ server

Simplified flow, server → client (the app's `state::sync::sync_all`, triggered
by `runSync()` in `app/layers/shared/stores/sync-queue.ts`). Each table syncs
one at a time.

1. **Trigger** — a client sync starts. It runs once per contract table.

2. **Ask first** — for each table, the client sends what *it* has
   (`local_delta()`); the server merges and replies. One round-trip does both
   upload and download — there is no separate pull call.

3. **Server auth** — `POST /sync` with `Authorization: Bearer <JWT>` +
   `x-device-id`. The server decodes the JWT → `user_identifier` = **tenant**.
   Which device belongs to which user comes entirely from the token + header;
   no login or account-DB lookup.

4. **Server merge (per tenant)** — in a PG transaction it inserts the sent
   cells into that tenant's `{table}_crdt` log (last-writer-wins by lamport
   clock), stamping each with a `seq`.

5. **Server feeds back** — it reads the tenant's whole change log for the
   table, filtered by the device's **cursor** (only what this device hasn't
   seen, including other devices' edits), and returns a `SyncPayload` + `has_more`.

6. **Client applies** — `complete_sync(sent, response)`:
   - acks the sent cells (marks them clean locally, so they are not re-uploaded),
   - writes the received cells into the local SQLite CRDT store (`{table}` +
     `{table}_crdt`) — same LWW merge; the local clock advances.

7. **Page loop** — repeat while `has_more` (bounded by the page budget).

8. **Mirror down to the app** — after all tables, `apply_all` reverses the
   mirror: received `RowChange`s are upserted into the sea-orm models and
   written into the legacy app tables (`todo`, `notes`, …). Tombstoned server
   rows are removed from the app tables.

9. **UI sees it** — the app reads those same legacy tables, so edits from other
   devices just appear.

## Where old data comes from (backfill)

Data that already exists in your legacy tables (`todo`, `notes`, …) lands in
the sync system through **backfill** — nothing old is uploaded until it does.

- On every launch, the app checks each store table
  (`backfill_missing` → `SqliteClient::is_empty`).
- If a store table is empty but its legacy table has rows, it mirrors every
  legacy row into the store (`mirror_*` → same insert/set path as live writes),
  tagged with this device's id and a fresh lamport clock, marked **unsent**.
- Those seeded rows then show up in the very first `local_delta()`, so the
  round-trip above uploads your entire existing dataset, and the server merges
  it under the token's tenant.
- It's idempotent and self-healing: tables that were already seeded are left
  untouched, and a swapped-in/replaced database re-seeds automatically.
  `just reset-backfill` force-cleans the store tables so the next launch
  re-mirrors everything.

Caveat: seeded rows only leave the app when a sync succeeds with a valid token;
otherwise they sit pending in the store and go out on a later successful sync.

## Why it converges

Everything is per-cell last-writer-wins in the CRDT log, and the client
(SQLite) and server (PostgreSQL) keep the same log shape, so merging is
symmetric and any device can replay the same steps. The app tables are a
read-path projection of the store; `apply_all` is what pushes server rows into
the legacy tables.