<script setup lang="ts">
import { kFab } from "konsta/vue";
import { useTodoStore } from "@shared/stores/todo";
import TodoCard from "@shared/components/todo/todo-card.vue";
import EmptyState from "@shared/components/app/EmptyState.vue";

definePageMeta({ name: "Tasks" });

const todoStore = useTodoStore();
const { notify } = useAppNotification();

const showCreatePopup = ref(false);

const fabColors = {
  bgIos: "bg-primary-500 dark:bg-primary-600",
  bgMaterial: "bg-primary-500 dark:bg-primary-600",
  textIos: "text-white",
  textMaterial: "text-white",
};

onMounted(() => {
  todoStore.fetchTodos();
});

function handleCreated() {
  notify({ message: "Task created", type: "success" });
}
</script>

<template>
  <div>
    <!-- Create task FAB -->
    <div
      v-if="todoStore.todos.length !== 0"
      class="fixed bottom-20 right-5 z-40"
    >
      <kFab
        component="button"
        aria-label="Add task"
        :colors="fabColors"
        @click="showCreatePopup = true"
      >
        <template #icon>
          <UIcon name="heroicons:plus" class="size-6" />
        </template>
      </kFab>
    </div>

    <!-- Loading -->
    <div v-if="todoStore.loading" class="flex flex-col gap-2">
      <USkeleton v-for="i in 4" :key="i" class="h-16 rounded-lg" />
    </div>

    <!-- Empty state: no todos at all -->
    <div
      v-else-if="todoStore.todos.length === 0"
      class="flex flex-col items-center justify-center py-20 text-center"
    >
      <EmptyState
        title="No task yet"
        description="Create your first task to get started."
        icon="ri:calendar-todo-line"
        action-label="create task"
        @action="showCreatePopup = true"
      />
    </div>

    <!-- Todo list -->
    <div v-else class="flex flex-col gap-2">
      <TodoCard
        v-for="todo in todoStore.todos"
        :key="todo.identifier"
        :todo="todo"
        @toggle="(id, done) => todoStore.toggleDone(id, done)"
        @delete="(id) => todoStore.deleteTodo(id)"
      />
    </div>

    <!-- Create task popup -->
    <TodoCreatePopup
      v-model:open="showCreatePopup"
      @created="handleCreated"
    />
  </div>
</template>
