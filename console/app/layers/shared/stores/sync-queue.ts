import { useAuthStore } from "@shared/stores/auth";
import { invoke } from "@shared/utils/invoke";
import { useNetwork } from "@vueuse/core";
import { defineStore } from "pinia";
import { ref } from "vue";

export const useSyncQueueStore = defineStore("sync_queue_store", () => {
  const { isOnline } = useNetwork();
  const runningSync = ref(false);

  async function runSync() {
    if (runningSync.value || !isOnline.value) return;
    runningSync.value = true;
    try {
      const auth = useAuthStore();
      // Logged-in users pass their JWT; guest mode falls back to the
      // development token (`LUNAR_DEV_TOKEN`) resolved on the Rust side.
      await invoke("sync_all", {
        // token: auth.isAuthenticated ? auth.accessToken : undefined,
        token:  `eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiI0MjA1MGU2Yy05NDQ4LTQyMzctYjBlYS04YzJjYTNkZWZjNTgiLCJpYXQiOjE3ODc5NTkyMTYsImV4cCI6MTc4ODU2NDAxNiwiZW1haWwiOiJkZXZAdGVzdC50ZXN0IiwidXNlcl9pZGVudGlmaWVyIjoiNDIwNTBlNmMtOTQ0OC00MjM3LWIwZWEtOGMyY2EzZGVmYzU4In0.5dyttXN643ojyDPDDfZGOLKhGUCqZWAMvhXLRZjNMM0`,
      });
    } catch (error) {
      console.error("[sync] sync_all failed", error);
    } finally {
      runningSync.value = false;
    }
  }

  return { isOnline, runningSync, runSync };
});
