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
const errors = reactive({ name: "", description: "" });
const loading = ref(false);
const submitError = ref("");

function resetForm() {
  Object.assign(form, { name: "", description: "" });
  Object.assign(errors, { name: "", description: "" });
  submitError.value = "";
}

function requestClose() {
  resetForm();
  emit("update:open", false);
}

function validate(): boolean {
  errors.name = form.name.trim() ? "" : "Name is required";
  errors.description = form.description.trim() ? "" : "Description is required";
  return !errors.name && !errors.description;
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
    <div class="bg-gray-50 dark:bg-app-dark-800 rounded-t-2xl px-4 pb-8 pt-3">
      <div class="mx-auto mb-3 h-1 w-10 rounded-full bg-gray-300 dark:bg-gray-600" />

      <div class="mb-2 flex items-center justify-between">
        <AppPageTitle>{{ title }}</AppPageTitle>
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

      <p class="mb-4 text-sm text-gray-500 dark:text-gray-400">
        {{ description }}
      </p>

      <form class="flex flex-col gap-4" @submit.prevent="handleSubmit">
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
          label="Description"
          type="text"
          size="sm"
          name="workspace-description"
          placeholder="Organize files and tasks"
          :disabled="loading"
        />
        <p v-if="errors.description" class="-mt-3 text-xs text-red-500">
          {{ errors.description }}
        </p>

        <p v-if="submitError" class="text-sm text-red-500">
          {{ submitError }}
        </p>

        <div class="flex gap-2 pt-2 justify-between">
          <UButton
            color="error"
            variant="outline"
            :disabled="loading"
            class=""
            @click="requestClose"
            >Cancel</UButton
          >

          <UButton type="submit" :loading="loading">
            {{ submitLabel }}
          </UButton>
        </div>
      </form>
    </div>
  </kSheet>
</template>
