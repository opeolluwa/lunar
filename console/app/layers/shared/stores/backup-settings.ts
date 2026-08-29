import { defineStore } from "pinia";

type BackupProvider = "local" | "cloud";

interface BackupServerConfig {
  provider: BackupProvider;
}

const STRONGHOLD_KEY = "sync-server";

export const useBackupSettingsStore = defineStore("backup_settings", {
  state: () => ({
    initialized: false,
    provider: "local" as BackupProvider,
    savedConfig: null as BackupServerConfig | null,
  }),

  getters: {
    savedConfigExists: (state) => state.savedConfig !== null,
  },

  actions: {
    async init() {
      if (this.initialized) return;

      const stronghold = useStronghold();

      try {
        await stronghold.init(STRONGHOLD_KEY);
        this.initialized = true;
      } catch (error) {
        console.error("Failed to initialize stronghold:", error);
        return;
      }

      try {
        const config =
          await stronghold.getItem<BackupServerConfig>(STRONGHOLD_KEY);
        if (config) {
          this.savedConfig = config;
          this.provider = config.provider;
        }
      } catch (error) {
        console.error("Failed to load backup config:", error);
      }
    },

    async save() {
      const stronghold = useStronghold();

      const payload: BackupServerConfig = {
        provider: this.provider,
      };

      await stronghold.setItem(STRONGHOLD_KEY, payload);
      this.savedConfig = payload;
    },
  },
});
