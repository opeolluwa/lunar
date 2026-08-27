import type { CreateReminder, Reminder, UpdateReminder } from "lunar";
import { defineStore } from "pinia";
import { invoke } from "../utils/invoke";

export type { Reminder };

export type CreateReminderPayload = Partial<CreateReminder> & {
  title: string;
  remindAt: string;
};

export type UpdateReminderPayload = Partial<UpdateReminder>;

export const useReminderStore = defineStore("reminder_store", {
  state: () => ({
    reminders: [] as Reminder[],
    loading: false,
  }),

  getters: {
    upcomingReminders: (state) => {
      const now = new Date().toISOString();
      return state.reminders.filter((r) => r.remindAt > now);
    },
    recurringReminders: (state) => state.reminders.filter((r) => r.recurring),
    oneTimeReminders: (state) => state.reminders.filter((r) => !r.recurring),
  },

  actions: {
    async fetchReminders() {
      this.loading = true;

      try {
        this.reminders = await invoke<Reminder[]>("get_all_reminders", {
          meta: await getWorkspaceMeta(),
        });
      } finally {
        this.loading = false;
      }
    },

    async createReminder(payload: CreateReminderPayload): Promise<Reminder> {
      const created = await invoke<Reminder>("create_reminder", {
        reminder: payload,
        meta: await getWorkspaceMeta(),
      });

      this.reminders.unshift(created);
      return created;
    },

    async updateReminder(
      identifier: string,
      payload: UpdateReminderPayload,
    ): Promise<Reminder> {
      const updated = await invoke<Reminder>("update_reminder", {
        identifier,
        reminder: payload,
        meta: await getWorkspaceMeta(),
      });

      const idx = this.reminders.findIndex((r) => r.identifier === identifier);
      if (idx !== -1) this.reminders[idx] = updated;

      return updated;
    },

    async deleteReminder(identifier: string) {
      await invoke("delete_reminder", {
        identifier,
        meta: await getWorkspaceMeta(),
      });

      this.reminders = this.reminders.filter(
        (r) => r.identifier !== identifier,
      );
    },

    async duplicateReminder(
      recordIdentifier: string,
      previousWorkspaceIdentifier: string,
      targetWorkspaceIdentifier: string,
    ) {
      await invoke("duplicate_reminder", {
        recordIdentifier,
        previousWorkspaceIdentifier,
        targetWorkspaceIdentifier,
        meta: await getWorkspaceMeta(),
      });

      await this.fetchReminders();
    },

    async transferReminder(
      recordIdentifier: string,
      previousWorkspaceIdentifier: string,
      targetWorkspaceIdentifier: string,
    ) {
      await invoke("transfer_reminder", {
        recordIdentifier,
        previousWorkspaceIdentifier,
        targetWorkspaceIdentifier,
        meta: await getWorkspaceMeta(),
      });

      this.reminders = this.reminders.filter(
        (r) => r.identifier !== recordIdentifier,
      );
    },

    async fetchUnsynced() {
      try {
        const reminders = await invoke<Reminder[]>("get_unsynced_reminders");
        return reminders;
      } catch (error) {
        console.error("Error fetching unsynced reminders:", error);
        return [];
      }
    },

  },
});
