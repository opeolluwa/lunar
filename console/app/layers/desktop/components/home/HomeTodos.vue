<script setup lang="ts">
import type { TodoInterface } from "lunar";

const props = defineProps<{
  todos: TodoInterface[];
  loading: boolean;
}>();

const emit = defineEmits<{ delete: [id: string] }>();

const priorityOrder: Record<string, number> = {
  high: 0,
  medium: 1,
  low: 2,
};

const activeTodos = computed(() =>
  [...props.todos]
    .filter((t) => !t.done)
    .sort((a, b) => priorityOrder[a.priority] - priorityOrder[b.priority])
    .slice(0, 7),
);

const completedCount = computed(
  () => props.todos.filter((t) => t.done).length,
);

const progress = computed(() => {
  const total = props.todos.length;
  if (total === 0) return 0;
  return Math.round((completedCount.value / total) * 100);
});

const RING_R = 26;
const RING_C = computed(() => 2 * Math.PI * RING_R);
const ringOffset = computed(() => RING_C.value * (1 - progress.value / 100));

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
    class="bg-white dark:bg-gray-800/60 rounded-2xl border border-gray-200 dark:border-white/15 overflow-hidden flex flex-col h-full"
  >
    <div
      class="flex items-center gap-3 px-5 py-4 border-b border-gray-100 dark:border-white/10"
    >
      <div class="relative size-15 shrink-0">
        <svg class="size-15 -rotate-90" viewBox="0 0 68 68">
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
          class="absolute inset-0 flex items-center justify-center text-xs font-bold text-gray-700 dark:text-gray-300"
        >
          {{ progress }}%
        </span>
      </div>

      <div class="flex-1 min-w-0">
        <h2 class="text-sm font-semibold text-gray-800 dark:text-gray-300/70">
          Active Todos
        </h2>
        <p class="text-xs text-gray-400 mt-0.5">
          {{ completedCount }} of {{ todos.length }} complete
        </p>
      </div>
    </div>

    <div class="flex-1">
      <div
        v-if="loading"
        class="flex items-center gap-2 px-5 py-6 text-gray-400 text-sm"
      >
        <UIcon name="heroicons:arrow-path" class="size-4 animate-spin" />
        Loading…
      </div>

      <div
        v-else-if="activeTodos.length === 0"
        class="flex flex-col items-center justify-center py-12 text-center"
      >
        <div
          class="p-2 flex justify-center items-center rounded-full bg-gray-100 dark:bg-gray-800"
        >
          <UIcon
            name="heroicons:check-circle"
            class="size-6 text-gray-400 dark:text-gray-500"
          />
        </div>
        <p class="mt-3 text-xs font-medium text-gray-600 dark:text-gray-400">
          No active todos
        </p>
        <p class="mt-0.5 text-xs text-gray-400 dark:text-gray-500">
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
          class="group flex items-center gap-3 px-5 py-3 hover:bg-gray-50 dark:hover:bg-gray-800/40 transition-colors"
        >
          <span
            class="size-1.5 rounded-full shrink-0"
            :class="priorityDot[todo.priority]"
          />
          <div class="flex-1 min-w-0">
            <p class="text-sm text-gray-800 dark:text-gray-200 truncate">
              {{ todo.title }}
            </p>
            <p v-if="todo.dueDate" class="text-xs text-gray-400 mt-0.5">
              Due {{ formatDate(todo.dueDate) }}
            </p>
          </div>
          <span
            class="text-xs text-gray-400 dark:text-gray-500 capitalize shrink-0"
          >
            {{ todo.priority }}
          </span>
          <button
            class="opacity-0 group-hover:opacity-100 transition-opacity text-gray-300 hover:text-red-400 dark:text-gray-700 dark:hover:text-red-400 shrink-0"
            @click="emit('delete', todo.identifier)"
          >
            <UIcon name="heroicons:trash" class="size-3.5" />
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
