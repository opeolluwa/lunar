import { gql } from "@apollo/client";
import { useBookmarkStore } from "@shared/stores/bookmarks";
import { useNoteStore } from "@shared/stores/notes";
import { useNotificationStore } from "@shared/stores/notifications";
import { useRecycleBinStore } from "@shared/stores/recycle-bin";
import { useReminderStore } from "@shared/stores/reminder";
import { useSnippetStore } from "@shared/stores/snippets";
import { useTodoStore } from "@shared/stores/todo";
import { useUserPreferenceStore } from "@shared/stores/workspace-profile";
import { useWorkspacesStore } from "@shared/stores/workspaces";
import { useNetwork } from "@vueuse/core";
import { defineStore } from "pinia";
import { ref } from "vue";

export const useSyncQueueStore = defineStore("sync_queue_store", () => {
  const { isOnline } = useNetwork();
  const runningSync = ref(false);

  async function preflightCheck(name: string) {
    const query = gql`
      mutation PreflightCheck($name: String!) {
        preflight(name: $name)
      }
    `;

    const variables = { name };

    const { mutate } = useMutation(query, { variables });
    const data = await mutate();
    console.log("Preflight check response:", data);
  }

  async function runSync() {
    if (runningSync.value || !isOnline.value) return;
    // runningSync.value = true;
  }

  return { isOnline, runningSync, preflightCheck, runSync };
});
