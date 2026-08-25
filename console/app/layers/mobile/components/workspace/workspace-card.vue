<script setup lang="ts">
import type { Workspace } from "@shared/stores/workspaces";
import MetaControls from "../meta/meta-controls.vue";

defineProps<{ workspace: Workspace }>();

const emit = defineEmits<{
  delete: [identifier: string];
  edit: [identifier: string];
  toggleHidden: [identifier: string];
  setDefault: [identifier: string];
  toggleSecured: [identifier: string];
}>();

function formatDate(iso: string) {
  return new Date(iso).toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
  });
}
</script>

<template>
  <div
    class="group bg-white dark:bg-gray-800 rounded-lg p-4 border border-gray-100 dark:border-gray-700 hover:shadow-sm transition-shadow flex items-center gap-4"
  >
    <UIcon name="heroicons:briefcase" class="size-5 text-primary-500 shrink-0" />
    <div class="flex-1 min-w-0">
      <div class="flex items-center gap-2">
        <h3
          class="text-sm font-medium text-gray-800 dark:text-gray-200 truncate"
        >
          {{ workspace.name }}
        </h3>
        <span
          v-if="workspace.isDefault"
          class="px-1.5 py-0.5 rounded text-[10px] font-medium bg-primary-100 dark:bg-primary-900 text-primary-600 dark:text-primary-300 shrink-0"
        >
          default
        </span>
        <span
          v-if="workspace.isHidden"
          class="px-1.5 py-0.5 rounded text-[10px] font-medium bg-gray-100 dark:bg-gray-700 text-gray-500 dark:text-gray-400 shrink-0"
        >
          hidden
        </span>
        <span
          v-if="workspace.isSecured"
          class="px-1.5 py-0.5 rounded text-[10px] font-medium bg-amber-100 dark:bg-amber-900 text-amber-600 dark:text-amber-300 shrink-0"
        >
          secured
        </span>
      </div>
      <div class="text-xs text-gray-400 truncate block">
        {{ workspace.description }}
      </div>
    </div>
    <p class="text-xs text-gray-400 shrink-0 hidden sm:block">
      {{ formatDate(workspace.createdAt) }}
    </p>
    <div
      class="opacity-100 md:opacity-0 md:group-hover:opacity-100 transition-opacity flex items-center gap-1"
    >
      <MetaControls
        item-name="workspace"
        :show-edit="true"
        :show-duplicate="false"
        :show-transfer="false"
        :show-set-default="true"
        :is-default="workspace.isDefault"
        :show-toggle-hidden="true"
        :is-hidden="workspace.isHidden"
        :show-toggle-secured="true"
        :is-secured="workspace.isSecured"
        @edit-record="emit('edit', workspace.identifier)"
        @delete-record="emit('delete', workspace.identifier)"
        @set-default-record="emit('setDefault', workspace.identifier)"
        @toggle-hidden-record="emit('toggleHidden', workspace.identifier)"
        @toggle-secured-record="emit('toggleSecured', workspace.identifier)"
      />
    </div>
  </div>
</template>
