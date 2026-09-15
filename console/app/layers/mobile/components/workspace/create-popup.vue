<script setup lang="ts">
import { kSheet } from "konsta/vue";
import { useUserPreferenceStore } from "@shared/stores/workspace-profile";
import { useWorkspacesStore } from "@shared/stores/workspaces";
import type { Workspace } from "@shared/stores/workspaces";

withDefaults(
  defineProps<{
    open: boolean;
    title?: string;
    description?: string;
    submitLabel?: string;
  }>(),
  {
    title: "New Workspace",
    description:
      "Workspaces allow you to organize your notes, tasks and bookmarks.",
    submitLabel: "Submit",
  },
);

const emit = defineEmits<{
  "update:open": [value: boolean];
  created: [
    workspace: Workspace,
    initialProfile: { firstName: string; lastName: string; email: string },
  ];
}>();

const workspaceStore = useWorkspacesStore();
const preferenceStore = useUserPreferenceStore();

const form = reactive({ name: "", description: "" });
const errors = reactive({ name: "" });
const loading = ref(false);
const submitError = ref("");

function resetForm() {
  Object.assign(form, { name: "", description: "" });
  Object.assign(errors, { name: "" });
  submitError.value = "";
}

function requestClose() {
  resetForm();
  emit("update:open", false);
}

function validate(): boolean {
  errors.name = form.name.trim() ? "" : "Name is required";
  return !errors.name;
}

async function handleSubmit() {
  if (!validate()) return;
  loading.value = true;
  submitError.value = "";
  try {
    const prefSnapshot = preferenceStore.preference;
    const created = await workspaceStore.createWorkspace({
      name: form.name.trim(),
      description: form.description.trim(),
    });
    resetForm();
    emit("update:open", false);
    emit("created", created, {
      firstName: prefSnapshot?.firstName ?? "",
      lastName: prefSnapshot?.lastName ?? "",
      email: prefSnapshot?.email ?? "",
    });
  } catch (e) {
    console.error(e);
    submitError.value = "Failed to create workspace. Please try again.";
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <kSheet :opened="open" @backdropclick="requestClose">
    <div
      class="flex max-h-[85dvh] flex-col overflow-hidden rounded-t-[20px] bg-white pb-1 dark:bg-app-dark-800"
      style="padding-bottom: env(safe-area-inset-bottom, 0px)"
    >
      <div
        class="mx-auto my-3 h-1 w-9 shrink-0 rounded-full bg-gray-300 dark:bg-gray-600"
      />

      <div class="shrink-0 px-4 pb-5">
        <div class="flex items-center justify-between">
          <h2 class="text-lg font-semibold leading-6 text-gray-900 dark:text-white">
            {{ title }}
          </h2>
          <UButton
            size="md"
            color="neutral"
            variant="ghost"
            icon="heroicons:x-mark"
            aria-label="Close"
            :disabled="loading"
            class="-mr-2"
            @click="requestClose"
          />
        </div>
        <p class="pt-1 text-sm text-gray-500 dark:text-gray-400">
          {{ description }}
        </p>
      </div>

      <form
        class="flex min-h-0 flex-1 flex-col"
        @submit.prevent="handleSubmit"
      >
        <div
          class="flex-1 overflow-y-auto px-4 pb-4"
          style="-webkit-overflow-scrolling: touch"
        >
          <div class="flex flex-col gap-4">
            <AppInput
              v-model="form.name"
              label="Name"
              type="text"
              name="workspace-name"
              placeholder="Lunar"
              :disabled="loading"
            />
            <p v-if="errors.name" class="-mt-3 text-xs text-red-500">
              {{ errors.name }}
            </p>

            <AppInput
              v-model="form.description"
              label="Description (optional)"
              type="text"
              size="sm"
              name="workspace-description"
              placeholder="Organize files and tasks"
              :disabled="loading"
            />

            <p v-if="submitError" class="text-sm text-red-500">
              {{ submitError }}
            </p>
          </div>
        </div>

        <div
          class="flex shrink-0 items-center justify-end gap-3 px-4 pb-2 pt-3"
        >
          <UButton type="submit" :loading="loading">
            {{ submitLabel }}
          </UButton>
        </div>
      </form>
    </div>
  </kSheet>
</template>
