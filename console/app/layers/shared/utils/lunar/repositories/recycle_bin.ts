import type { CreateRecycleBinEntry, ItemType, RecycleBin } from "lunar";
import { BaseRepository, type RequestMeta } from "../base";

export type { CreateRecycleBinEntry };

const SOURCE_TABLES: Record<ItemType, string> = {
  note: "notes",
  todo: "todo",
  bookmark: "bookmark",
  snippet: "snippets",
  reminder: "reminder",
};

function snakeize(key: string): string {
  return key.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();
}

export class RecycleBinRepository extends BaseRepository {
  async store(
    payload: CreateRecycleBinEntry,
    meta?: RequestMeta,
  ): Promise<RecycleBin> {
    const m = this.requireMeta(meta);
    return this.mustRow<RecycleBin>(
      `INSERT INTO recycle_bin
         (identifier, item_id, item_type, payload, deleted_at, workspace_identifier)
       VALUES ($1, $2, $3, $4, $5, $6)
       RETURNING *`,
      [
        this.newUuid(),
        payload.itemId,
        payload.itemType,
        payload.payload,
        this.now(),
        m.workspaceIdentifier,
      ],
    );
  }

  async find_by_id(
    identifier: string,
    meta?: RequestMeta,
  ): Promise<RecycleBin | null> {
    const m = this.requireMeta(meta);
    return this.row<RecycleBin>(
      `SELECT * FROM recycle_bin WHERE identifier = $1 AND workspace_identifier = $2`,
      [identifier, m.workspaceIdentifier],
    );
  }

  async find_all(meta?: RequestMeta): Promise<RecycleBin[]> {
    const m = this.requireMeta(meta);
    return this.rows<RecycleBin>(
      `SELECT * FROM recycle_bin WHERE workspace_identifier = $1 ORDER BY deleted_at DESC`,
      [m.workspaceIdentifier],
    );
  }

  async find_by_item_type(
    item_type: ItemType,
    meta?: RequestMeta,
  ): Promise<RecycleBin[]> {
    const m = this.requireMeta(meta);
    return this.rows<RecycleBin>(
      `SELECT * FROM recycle_bin WHERE item_type = $1 AND workspace_identifier = $2 ORDER BY deleted_at DESC`,
      [item_type, m.workspaceIdentifier],
    );
  }

  async purge(identifier: string, meta?: RequestMeta): Promise<void> {
    const m = this.requireMeta(meta);
    await this.run(
      `DELETE FROM recycle_bin WHERE identifier = $1 AND workspace_identifier = $2`,
      [identifier, m.workspaceIdentifier],
    );
  }

  async purge_all(meta?: RequestMeta): Promise<void> {
    const m = this.requireMeta(meta);
    await this.run(`DELETE FROM recycle_bin WHERE workspace_identifier = $1`, [
      m.workspaceIdentifier,
    ]);
  }

  async restore(identifier: string, meta?: RequestMeta): Promise<void> {
    const m = this.requireMeta(meta);
    const entry = await this.find_by_id(identifier, m);
    if (!entry) throw new Error("recycle bin entry not found");

    const table = SOURCE_TABLES[entry.itemType];
    if (!table) throw new Error(`unsupported item type: ${entry.itemType}`);

    // `find_by_id` runs through `toCamelRow`, so the payload may already be
    // a parsed object rather than the raw JSON string that was stored.
    const raw =
      typeof entry.payload === "string"
        ? entry.payload
        : JSON.stringify(entry.payload);
    const record = JSON.parse(raw) as Record<string, unknown>;

    const columns: string[] = [];
    const params: unknown[] = [];
    for (const [key, value] of Object.entries(record)) {
      columns.push(snakeize(key));
      params.push(
        value !== null && typeof value === "object"
          ? JSON.stringify(value)
          : value,
      );
    }
    const placeholders = columns.map((_, i) => `$${i + 1}`).join(", ");
    await this.run(
      `INSERT INTO "${table}" (${columns.join(", ")}) VALUES (${placeholders})`,
      params,
    );

    await this.purge(identifier, m);
  }
}
