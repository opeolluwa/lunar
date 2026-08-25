<script setup lang="ts">
import { useTodoStore } from "@shared/stores/todo";

const todoStore = useTodoStore();

const priorityOrder: Record<"high" | "medium" | "low", number> = {
  high: 0,
  medium: 1,
  low: 2,
};

const activeTodos = computed(() =>
  [...todoStore.todos]
    .filter((t) => !t.done)
    .sort((a, b) => priorityOrder[a.priority] - priorityOrder[b.priority])
    .slice(0, 5),
);

const todoProgress = computed(() => {
  const total = todoStore.todos.length;
  if (total === 0) return 0;
  return Math.round((todoStore.completedTodos.length / total) * 100);
});

const RING_R = 26;
const RING_C = computed(() => 2 * Math.PI * RING_R);
const ringOffset = computed(
  () => RING_C.value * (1 - todoProgress.value / 100),
);

const priorityDot: Record<string, string> = {
  high: "bg-red-400",
  medium: "bg-amber-400",
  low: "bg-emerald-400",
};

function formatDate(iso: string) {
  return new Date(iso).toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
  });
}
</script>

<template>
  <div
    class="mt-5 overflow-hidden rounded-2xl border border-gray-200 bg-white dark:border-white/15 dark:bg-gray-800/60"
  >
    <div
      class="flex items-center gap-3 border-b border-gray-100 px-4 py-3 dark:border-white/10"
    >
      <div class="relative size-12 shrink-0">
        <svg class="size-12 -rotate-90" viewBox="0 0 68 68">
          <circle
            cx="34"
            cy="34"
            :r="RING_R"
            fill="none"
            stroke="currentColor"
            stroke-width="5"
            class="text-gray-100 dark:text-gray-800"
          />
          <circle
            cx="34"
            cy="34"
            :r="RING_R"
            fill="none"
            stroke="currentColor"
            stroke-width="5"
            stroke-linecap="round"
            class="text-primary-500 transition-all duration-700"
            :stroke-dasharray="RING_C"
            :stroke-dashoffset="ringOffset"
          />
        </svg>
        <span
          class="absolute inset-0 flex items-center justify-center text-[11px] font-bold text-gray-700 dark:text-gray-300"
        >
          {{ todoProgress }}%
        </span>
      </div>

      <div class="min-w-0 flex-1">
        <h2 class="text-sm font-semibold text-gray-800 dark:text-gray-300/70">
          Active Todos
        </h2>
        <p class="mt-0.5 text-[11px] text-gray-400">
          {{ todoStore.completedTodos.length }} of
          {{ todoStore.todos.length }} complete
        </p>
      </div>

      <NuxtLink
        to="/todo"
        class="text-xs font-medium text-primary-500 transition-colors hover:text-primary-600"
      >
        See all
      </NuxtLink>
    </div>

    <div
      v-if="todoStore.loading"
      class="flex items-center gap-2 px-4 py-6 text-sm text-gray-400"
    >
      <UIcon name="heroicons:arrow-path" class="size-4 animate-spin" />
      Loading…
    </div>

    <div
      v-else-if="activeTodos.length === 0"
      class="flex flex-col items-center justify-center py-8 text-center"
    >
      <div
        class="flex items-center justify-center rounded-full bg-gray-100 p-2 dark:bg-gray-800"
      >
        <UIcon
          name="heroicons:check-circle"
          class="size-5 text-gray-400 dark:text-gray-500"
        />
      </div>
      <p class="mt-2 text-xs font-medium text-gray-600 dark:text-gray-400">
        No active todos
      </p>
      <p class="mt-0.5 text-[11px] text-gray-400 dark:text-gray-500">
        Your pending tasks will appear here.
      </p>
    </div>

    <div
      v-else
      class="divide-y divide-gray-100 dark:divide-gray-700/60"
    >
      <div
        v-for="todo in activeTodos"
        :key="todo.identifier"
        class="flex items-center gap-3 px-4 py-3"
      >
        <span
          class="size-1.5 shrink-0 rounded-full"
          :class="priorityDot[todo.priority]"
        />
        <div class="min-w-0 flex-1">
          <p class="truncate text-sm text-gray-800 dark:text-gray-200">
            {{ todo.title }}
          </p>
          <p v-if="todo.dueDate" class="mt-0.5 text-[11px] text-gray-400">
            Due {{ formatDate(todo.dueDate) }}
          </p>
        </div>
        <span
          class="shrink-0 text-[11px] capitalize text-gray-400 dark:text-gray-500"
        >
          {{ todo.priority }}
        </span>
      </div>
    </div>
  </div>
</template>
