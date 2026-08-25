<script setup lang="ts">
import { kFab } from "konsta/vue";
import { useWorkspacesStore } from "@shared/stores/workspaces";
import WorkspaceCard from "@mobile/components/workspace/workspace-card.vue"
definePageMeta({ name: "Workspaces" });

const { notify } = useAppNotification();
const workspaceStore = useWorkspacesStore();

const showCreateModal = ref(false);

function handleCreated() {
  notify({ message: "Workspace created", type: "success" });
}

const secureTargetId = ref<string | null>(null);

function handleToggleSecured(identifier: string) {
  const ws = workspaceStore.workspaces?.find(
    (w) => w.identifier === identifier,
  );
  if (!ws) return;
  secureTargetId.value = identifier;
}

function closeSecureModal() {
  secureTargetId.value = null;
}

async function handleDelete(identifier: string) {
  await workspaceStore.deleteWorkspace(identifier);
}

async function handleSetDefault(identifier: string) {
  try {
    await workspaceStore.updateWorkspace(identifier, { isDefault: true });
    notify({ message: "Default workspace updated", type: "success" });
  } catch (e) {
    notify({
      message: (e as Error).message || "Failed to update workspace",
      type: "error",
    });
  }
}

async function handleToggleHidden(identifier: string) {
  const ws = workspaceStore.workspaces?.find(
    (w) => w.identifier === identifier,
  );
  if (!ws) return;
  try {
    await workspaceStore.updateWorkspace(identifier, {
      isHidden: !ws.isHidden,
    });
    notify({
      message: ws.isHidden ? "Workspace is now visible" : "Workspace hidden",
      type: "success",
    });
  } catch (e) {
    notify({
      message: (e as Error).message || "Failed to update workspace",
      type: "error",
    });
  }
}

const editingId = ref<string | null>(null);
const editName = ref("");
const editDescription = ref("");
const editSubmitting = ref(false);

function handleEdit(identifier: string) {
  const ws = workspaceStore.workspaces?.find(
    (w) => w.identifier === identifier,
  );
  if (!ws) return;
  editingId.value = identifier;
  editName.value = ws.name;
  editDescription.value = ws.description;
}

function closeEdit() {
  editingId.value = null;
  editName.value = "";
  editDescription.value = "";
}

async function submitEdit() {
  if (!editingId.value) return;
  editSubmitting.value = true;
  try {
    await workspaceStore.updateWorkspace(editingId.value, {
      name: editName.value.trim() || undefined,
      description: editDescription.value.trim(),
    });
    notify({ message: "Workspace updated", type: "success" });
    closeEdit();
  } catch (e) {
    notify({
      message: (e as Error).message || "Failed to update workspace",
      type: "error",
    });
  } finally {
    editSubmitting.value = false;
  }
}

const workspaces = computed(() => workspaceStore.workspaces ?? []);
</script>

<template>
  <div class="flex flex-col gap-4 mt-4">
    <h2 class="text-sm font-semibold text-gray-700 dark:text-gray-200">
      Workspaces
    </h2>

    <AppEmptyState
      v-if="!workspaces.length && !workspaceStore.loading"
      icon="heroicons:briefcase"
      title="No workspaces yet"
      description="Create a workspace to organize your notes, tasks and bookmarks."
      action-label="Create workspace"
      @action="showCreateModal = true"
    />

    <div
      v-for="workspace in workspaces"
      :key="workspace.identifier"
      class="cursor-pointer"
    >
      <WorkspaceCard
        :workspace="workspace"
        @delete="handleDelete"
        @edit="handleEdit"
        @toggle-hidden="handleToggleHidden"
        @set-default="handleSetDefault"
        @toggle-secured="handleToggleSecured"
      />
    </div>

    <!-- Edit modal -->
    <UModal :open="!!editingId" @close="closeEdit">
      <template #content>
        <div class="p-6 flex flex-col gap-4">
          <h3 class="text-sm font-semibold text-gray-800 dark:text-gray-200">
            Edit workspace
          </h3>
          <UFormField label="Name">
            <UInput
              v-model="editName"
              placeholder="Workspace name"
              class="w-full"
              :disabled="editSubmitting"
            />
          </UFormField>
          <UFormField label="Description">
            <UInput
              v-model="editDescription"
              placeholder="Short description"
              class="w-full"
              :disabled="editSubmitting"
            />
          </UFormField>
          <div class="flex items-center gap-2 mt-2">
            <UButton
              size="sm"
              :loading="editSubmitting"
              :disabled="!editName.trim()"
              @click="submitEdit"
            >
              Save
            </UButton>
            <UButton
              variant="ghost"
              size="sm"
              :disabled="editSubmitting"
              @click="closeEdit"
            >
              Cancel
            </UButton>
          </div>
        </div>
      </template>
    </UModal>
    <!-- Secure workspace modal -->
    <WorkspaceSecureModal
      :open="!!secureTargetId"
      :workspace-id="secureTargetId"
      @update:open="closeSecureModal"
    />

    <!-- Create workspace popup -->
    <WorkspaceCreatePopup
      v-model:open="showCreateModal"
      title="New workspace"
      description="Workspaces allow you to organize your notes, tasks and bookmarks."
      submit-label="Continue"
      @created="handleCreated"
    />

    <kFab
      v-if="!showCreateModal"
      component="button"
      aria-label="New workspace"
      class="absolute bottom-24 right-7 z-[60] md:hidden"
      :colors="{
        bgIos: 'bg-primary-500 dark:bg-primary-600',
        bgMaterial: 'bg-primary-500 dark:bg-primary-600',
        textIos: 'text-white',
        textMaterial: 'text-white',
      }"
      @click="showCreateModal = true"
    >
      <template #icon>
        <UIcon name="heroicons:plus" class="size-6" />
      </template>
    </kFab>
  </div>
</template>
