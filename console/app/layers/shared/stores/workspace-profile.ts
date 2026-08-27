import type {
  CreateWorkspaceProfile,
  UpdateWorkspaceProfile,
  WorkspaceProfiles
} from "lunar";
import { defineStore } from "pinia";
import { invoke } from "../utils/invoke";
import { getWorkspaceMeta } from "#imports";

export const useUserPreferenceStore = defineStore("user_preference_store", {
  state: () => ({
    preference: null as WorkspaceProfiles | null,
    loading: false,
  }),

  getters: {
    fullName: (state) =>
      state.preference
        ? `${state.preference.firstName} ${state.preference.lastName}`.trim()
        : "",
  },

  actions: {
    async fetchPreference() {
      this.loading = true;
      try {
        this.preference = await invoke<WorkspaceProfiles | null>(
          "get_workspace_profile",
          {
            meta: await getWorkspaceMeta(),
          },
        );
      } catch (error) {
        console.error("[preferences] failed to fetch", error);
      } finally {
        this.loading = false;
      }
    },

    async createPreference(
      payload: CreateWorkspaceProfile,
    ): Promise<WorkspaceProfiles> {
      const created = await invoke<WorkspaceProfiles>(
        "create_workspace_profile",
        {
          profile: payload,
          meta: await getWorkspaceMeta(),
        },
      );
      this.preference = created;
      return created;
    },

    async updatePreference(
      payload: Partial<UpdateWorkspaceProfile>,
    ): Promise<WorkspaceProfiles> {
      if (!this.preference) {
        return this.createPreference(payload as CreateWorkspaceProfile);
      }
      const updated = await invoke<WorkspaceProfiles>(
        "update_workspace_profile",
        {
          identifier: this.preference.identifier,
          profile: payload,
          meta: await getWorkspaceMeta(),
        },
      );
      this.preference = updated;
      return updated;
    },

    async fetchUnsynced() {
      try {
        const userPreferences = await invoke<WorkspaceProfiles[]>(
          "get_unsynced_workspace_profiles",
        );
        return userPreferences;
      } catch (error) {
        console.error("Error fetching unsynced user preferences:", error);
        return [];
      }
    },

    async syncUpstream() {},

    async clearQueue(identifiers: string[]) {
      await invoke("clear_synced_workspace_profiles", { identifiers });
    },
  },
  persist: true,
});
