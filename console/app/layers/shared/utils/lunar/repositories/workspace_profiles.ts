import type {
  CreateWorkspaceProfile,
  UpdateWorkspaceProfile,
  WorkspaceProfiles,
} from "lunar";
import { BaseRepository, type RequestMeta } from "../base";

export type { CreateWorkspaceProfile, UpdateWorkspaceProfile };

const COLUMNS = [
  "first_name",
  "last_name",
  "profile_picture",
  "workspace_identifier",
  "created_at",
  "updated_at",
];

export class WorkspaceProfileRepository extends BaseRepository {
  async create(
    payload: CreateWorkspaceProfile,
    meta?: RequestMeta,
  ): Promise<WorkspaceProfiles> {
    const m = this.requireMeta(meta);
    const now = this.now();
    return this.mustRow<WorkspaceProfiles>(
      `INSERT INTO workspace_profiles
         (identifier, first_name, last_name, profile_picture, created_at, updated_at, workspace_identifier)
       VALUES ($1, $2, $3, $4, $5, $6, $7)
       RETURNING *`,
      [
        this.newUuid(),
        payload.firstName,
        payload.lastName,
        payload.profilePicture ?? null,
        now,
        now,
        m.workspaceIdentifier,
      ],
    );
  }

  async get(meta?: RequestMeta): Promise<WorkspaceProfiles | null> {
    const m = this.requireMeta(meta);
    return this.row<WorkspaceProfiles>(
      `SELECT * FROM workspace_profiles WHERE workspace_identifier = $1`,
      [m.workspaceIdentifier],
    );
  }

  async update(
    identifier: string,
    payload: UpdateWorkspaceProfile,
    meta?: RequestMeta,
  ): Promise<WorkspaceProfiles> {
    const m = this.requireMeta(meta);
    const sets: string[] = ["updated_at = $2"];
    const params: unknown[] = [identifier, this.now()];
    let idx = 3;

    if (payload.firstName !== undefined) {
      sets.push(`first_name = $${idx++}`);
      params.push(payload.firstName);
    }
    if (payload.lastName !== undefined) {
      sets.push(`last_name = $${idx++}`);
      params.push(payload.lastName);
    }
    if (payload.profilePicture !== undefined) {
      sets.push(`profile_picture = $${idx++}`);
      params.push(payload.profilePicture);
    }

    const row = await this.row<WorkspaceProfiles>(
      `UPDATE workspace_profiles SET ${sets.join(", ")} WHERE identifier = $1 AND workspace_identifier = $${idx}
       RETURNING *`,
      [...params, m.workspaceIdentifier],
    );
    if (!row) throw new Error("workspace profile not found");
    return row;
  }

  async transfer_record(
    record_identifier: string,
    previous_workspace_identifier: string,
    target_workspace_identifier: string,
  ): Promise<void> {
    await this.transferWorkspaceRecord(
      "workspace_profiles",
      record_identifier,
      previous_workspace_identifier,
      target_workspace_identifier,
    );
  }

  async duplicate_record(
    record_identifier: string,
    previous_workspace_identifier: string,
    target_workspace_identifier: string,
  ): Promise<void> {
    await this.duplicateWorkspaceRecord(
      "workspace_profiles",
      record_identifier,
      previous_workspace_identifier,
      target_workspace_identifier,
      COLUMNS,
    );
  }

  async record_exists_in_workspace(
    record_identifier: string,
    workspace_identifier: string,
  ): Promise<boolean> {
    return this.recordExistsInWorkspace(
      "workspace_profiles",
      record_identifier,
      workspace_identifier,
    );
  }
}
