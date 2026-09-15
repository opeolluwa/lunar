<script setup lang="ts">
import { useTodoStore } from "@shared/stores/todo";
import { useWorkspacesStore } from "@shared/stores/workspaces";
import type { Todo } from "@shared/stores/todo";
import MetaControls from "../meta/meta-controls.vue";

const { todo } = defineProps<{
  todo: Todo;
}>();

const emit = defineEmits<{
  toggle: [identifier: string, done: boolean];
  edit: [identifier: string];
  delete: [identifier: string];
}>();

const workspaceStore = useWorkspacesStore();
const currentWorkspaceId = computed(() => workspaceStore.activeWorkspaceId);
const todoStore = useTodoStore();
const handleDuplicate = async (targetWorkspaceId: string) => {
  await todoStore.duplicateTodo(
    todo.identifier,
    currentWorkspaceId.value,
    targetWorkspaceId,
  );
};

const handleTransfer = async (targetWorkspaceId: string) => {
  await todoStore.transferTodo(
    todo.identifier,
    currentWorkspaceId.value,
    targetWorkspaceId,
  );
};
</script>

<template>
  <div
    class="bg-white dark:bg-gray-800 rounded-lg p-4 border border-gray-100 dark:border-gray-700 flex items-center gap-4 hover:shadow-sm transition-shadow group"
  >
    <UCheckbox
      :label="todo.title"
      :description="String(todo.description)"
      :v-model="todo.done"
      :ui="{
        label: todo.done ? 'line-through text-gray-400' : '',
        description: todo.done ? 'line-through text-gray-400' : '',
        root: 'w-full',
      }"
      @update:model-value="(e) => emit('toggle', todo.identifier, e)"
    />

    <div
      class="flex items-center gap-1 opacity-100 md:opacity-0 md:group-hover:opacity-100 transition-opacity"
    >
      <MetaControls
        item-name="todo"
        @edit-record="emit('edit', todo.identifier)"
        @delete-record="emit('delete', todo.identifier)"
        @duplicate-record="
          (targetWorkspaceId) => handleDuplicate(targetWorkspaceId)
        "
        @transfer-record="
          (targetWorkspaceId) => handleTransfer(targetWorkspaceId)
        "
      />
    </div>
  </div>
</template>
