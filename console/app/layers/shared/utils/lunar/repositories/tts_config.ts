import type {
  CreateTtsConfig,
  UpdateTtsConfig,
  TtsConfig,
} from "lunar";
import { BaseRepository, type RequestMeta } from "../base";

export type { CreateTtsConfig, UpdateTtsConfig };

export class TtsConfigRepository extends BaseRepository {
  async getByUserIdentifier(
    userIdentifier: string | null | undefined,
    meta?: RequestMeta,
  ): Promise<TtsConfig | null> {
    const m = this.requireWorkspace(meta);
    const row = await this.row<TtsConfig>(
      `SELECT * FROM tts_config
        WHERE workspace_identifier = $1
          AND ($2::uuid IS NULL OR user_identifier = $2)
        LIMIT 1`,
      [m.workspaceIdentifier, userIdentifier ?? null],
    );
    return row;
  }

  async upsert(
    payload: CreateTtsConfig,
    meta?: RequestMeta,
  ): Promise<TtsConfig> {
    const m = this.requireWorkspace(meta);
    const existing = await this.getByUserIdentifier(payload.userIdentifier, meta);
    if (existing) {
      return this.mustRow<TtsConfig>(
        `UPDATE tts_config
            SET language = $1,
                voice_actor = $2,
                rate = $3,
                pitch = $4,
                volume = $5,
                updated_at = $6
          WHERE identifier = $7
          RETURNING *`,
        [
          payload.language,
          payload.voiceActor,
          payload.rate,
          payload.pitch,
          payload.volume,
          this.now(),
          existing.identifier,
        ],
      );
    }
    return this.mustRow<TtsConfig>(
      `INSERT INTO tts_config
         (identifier, workspace_identifier, user_identifier, language, voice_actor, rate, pitch, volume, created_at, updated_at)
       VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
       RETURNING *`,
      [
        this.newUuid(),
        m.workspaceIdentifier,
        payload.userIdentifier ?? null,
        payload.language,
        payload.voiceActor,
        payload.rate,
        payload.pitch,
        payload.volume,
        this.now(),
        this.now(),
      ],
    );
  }
}
