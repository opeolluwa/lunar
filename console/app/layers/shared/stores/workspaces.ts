import type { CreateWorkspace, UpdateWorkspace, Workspaces } from "lunar";
import { defineStore } from "pinia";
import { invoke } from "@shared/utils/invoke";
import { useNoteStore } from "./notes";
import { useTodoStore } from "./todo";
import { useBookmarkStore } from "./bookmarks";
import { useRecycleBinStore } from "./recycle-bin";
import { useReminderStore } from "./reminder";
import { useUserPreferenceStore } from "./workspace-profile";
import { useSnippetStore } from "./snippets";

export type Workspace = Workspaces;
export type CreateWorkspacePayload = CreateWorkspace;
export type UpdateWorkspacePayload = Partial<UpdateWorkspace>;

export const useWorkspacesStore = defineStore("workspaces_store", {
  state: () => ({
    workspaces: [] as Workspace[],
    loading: false,
    activeWorkspaceId: "" as string,
    /** Identifiers of secured workspaces the user has unlocked this session. */
    unlockedWorkspaceIds: [] as string[],
  }),

  getters: {
    currentWorkspace: (state) =>
      state.workspaces?.find((w) => w.identifier === state.activeWorkspaceId) ||
      null,

    visibleWorkspaces: (state) =>
      state.workspaces?.filter((w) => !w.isHidden) ?? [],

    isWorkspaceUnlocked: (state) => (identifier: string) =>
      !state.workspaces?.find((w) => w.identifier === identifier)?.isSecured ||
      state.unlockedWorkspaceIds.includes(identifier),

    isCurrentWorkspaceLocked: (state) => {
      const current = state.workspaces?.find(
        (w) => w.identifier === state.activeWorkspaceId,
      );
      return (
        !!current?.isSecured &&
        !state.unlockedWorkspaceIds.includes(state.activeWorkspaceId)
      );
    },
  },

  actions: {
    async fetchWorkspaces() {
      this.loading = true;
      try {
        this.workspaces = (await invoke<Workspace[]>("list_workspaces")) ?? [];
        if (!this.activeWorkspaceId && this.workspaces.length > 0) {
          // Prefer the default workspace on first load
          const defaultWs = this.workspaces.find((w) => w.isDefault);
          this.activeWorkspaceId = (defaultWs ?? this.workspaces[0]).identifier;
        }
      } catch (error) {
        console.error("[workspaces] failed to fetch", error);
      } finally {
        this.loading = false;
      }
    },

    async createWorkspace(payload: CreateWorkspacePayload): Promise<Workspace> {
      const created = await invoke<Workspace>("create_workspace", {
        workspace: payload,
      });
      this.workspaces.push(created);
      this.activeWorkspaceId = created.identifier;
      return created;
    },

    async updateWorkspace(
      identifier: string,
      payload: UpdateWorkspacePayload,
    ): Promise<Workspace> {
      const updated = await invoke<Workspace>("update_workspace", {
        identifier,
        workspace: payload,
      });
      const idx = this.workspaces.findIndex((w) => w.identifier === identifier);
      if (idx !== -1) this.workspaces[idx] = updated;
      return updated;
    },

    async deleteWorkspace(identifier: string): Promise<void> {
      const { notify } = useAppNotification();
      try {
        await invoke<Workspace>("delete_workspace", {
          identifier,
          meta: await getWorkspaceMeta(),
        });

        await this.fetchWorkspaces();
        notify({
          message: "Workspace deleted",
          type: "success",
        });
      } catch (error) {
        notify({
          message:
            (error as unknown as Error).message || "Failed to delete workspace",
          type: "error",
        });
      }
    },

    async setActiveWorkspace(identifier: string) {
      if (this.workspaces.find((w) => w.identifier === identifier)) {
        this.activeWorkspaceId = identifier;
      } else {
        console.warn("Workspace not found:", identifier);
      }

      const noteStore = useNoteStore();
      const todoStore = useTodoStore();
      const bookmarksStore = useBookmarkStore();
      const recycleBinStore = useRecycleBinStore();
      const reminderStore = useReminderStore();
      const userPreferenceStore = useUserPreferenceStore();
      const snippetsStore = useSnippetStore();

      await Promise.all([
        noteStore.fetchNotes(),
        noteStore.fetchRecentNotes(),
        todoStore.fetchTodos(),
        bookmarksStore.fetchBookmarks(),
        recycleBinStore.fetchEntries(),
        reminderStore.fetchReminders(),
        userPreferenceStore.fetchPreference(),
        snippetsStore.fetchSnippets(),
      ]);
    },

    async verifyWorkspacePassword(
      identifier: string,
      password: string,
    ): Promise<boolean> {
      return invoke<boolean>("verify_workspace_password", {
        identifier,
        password,
      });
    },

    unlockWorkspace(identifier: string) {
      if (!this.unlockedWorkspaceIds.includes(identifier)) {
        this.unlockedWorkspaceIds.push(identifier);
      }
    },

    async fetchUnsynced() {
      try {
        const workspaces = await invoke<Workspace[]>("get_unsynced_workspaces");
        return workspaces;
      } catch (error) {
        console.error("Error fetching unsynced workspaces:", error);
        return [];
      }
    },
  },
  persist: {
    omit: ["unlockedWorkspaceIds"],
    afterHydrate: (ctx) => {
      if (!Array.isArray(ctx.store.workspaces)) ctx.store.workspaces = [];
    },
  },
});
