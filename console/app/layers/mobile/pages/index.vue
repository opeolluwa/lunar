<script setup lang="ts">
import { useNoteStore } from "@shared/stores/notes";
import { useBookmarkStore } from "@shared/stores/bookmarks";
import { useTodoStore } from "@shared/stores/todo";
import { useUserPreferenceStore } from "@shared/stores/workspace-preferences";
import { safeOpenUrl as openUrl } from "@shared/utils/safe-open-url";

definePageMeta({ layout: "default", name: "Home" });

const noteStore = useNoteStore();
const bookmarkStore = useBookmarkStore();
const todoStore = useTodoStore();
const userPreferenceStore = useUserPreferenceStore();

onMounted(() => {
  Promise.all([
    noteStore.fetchNotes(),
    bookmarkStore.fetchBookmarks(),
    todoStore.fetchTodos(),
    userPreferenceStore.fetchPreference(),
  ]);
});

// Live clock
const now = ref(new Date());
let clockTimer: ReturnType<typeof setInterval>;
onMounted(() => {
  clockTimer = setInterval(() => {
    now.value = new Date();
  }, 60_000);
});
onUnmounted(() => clearInterval(clockTimer));

const greeting = computed(() => {
  const h = now.value.getHours();
  if (h < 12) return "Good morning";
  if (h < 17) return "Good afternoon";
  return "Good evening";
});

const today = computed(() =>
  now.value.toLocaleDateString("en-US", {
    weekday: "long",
    month: "long",
    day: "numeric",
  }),
);

const firstName = computed(
  () => userPreferenceStore.preference?.firstName || "there",
);

// Todos
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

// Notes & bookmarks
const recentNotes = computed(() => noteStore.notes.slice(0, 3));
const recentBookmarks = computed(() => bookmarkStore.bookmarks.slice(0, 3));

function stripHtml(html: string) {
  return html
    .replace(/<[^>]*>/g, " ")
    .replace(/\s+/g, " ")
    .trim();
}

function formatDate(iso: string) {
  return new Date(iso).toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
  });
}

// Stats pills
const statPills = computed(() => [
  {
    label: "Notes",
    value: noteStore.notes.length,
    icon: "heroicons:document-text-solid",
    color: "text-violet-500",
    href: "/notes",
  },
  {
    label: "Bookmarks",
    value: bookmarkStore.bookmarks.length,
    icon: "heroicons:bookmark-solid",
    color: "text-primary-500",
    href: "/bookmarks",
  },
  {
    label: "Active todos",
    value: todoStore.activeTodos.length,
    icon: "heroicons:check-circle-solid",
    color: "text-emerald-500",
    href: "/todo",
  },
]);
</script>

<template>
  <div>
    <!-- ── Header ─────────────────────────────────────────────── -->
    <div
      class="relative -mx-6 -mt-8 overflow-hidden bg-linear-to-br from-primary-500/10 via-violet-400/5 to-transparent px-6 pt-7 pb-6 dark:from-primary-500/12 dark:via-violet-500/6 dark:to-transparent"
    >
      <div
        class="pointer-events-none absolute -top-10 right-0 size-52 rounded-full bg-primary-300/20 blur-3xl dark:bg-primary-500/10"
      />
      <div
        class="pointer-events-none absolute bottom-0 left-1/2 size-36 -translate-y-1/2 rounded-full bg-violet-300/15 blur-2xl dark:bg-violet-500/8"
      />

      <div class="relative">
        <p
          class="mb-1.5 text-xs font-semibold uppercase tracking-widest text-gray-400 dark:text-gray-500"
        >
          {{ today }}
        </p>
        <h1 class="text-3xl font-bold tracking-tight text-gray-900 dark:text-white">
          {{ greeting }}, {{ firstName }}
        </h1>
        <p class="mt-1.5 text-sm text-gray-500 dark:text-gray-400">
          <template v-if="todoStore.activeTodos.length > 0">
            <strong class="text-gray-700 dark:text-gray-300">{{
              todoStore.activeTodos.length
            }}</strong>
            active
            {{ todoStore.activeTodos.length === 1 ? "todo" : "todos" }} today.
          </template>
          <template v-else> You're all caught up. </template>
        </p>
      </div>

      <!-- Stat pills -->
      <div class="relative mt-5 flex flex-wrap items-center gap-2">
        <NuxtLink
          v-for="s in statPills"
          :key="s.label"
          :to="s.href"
          class="flex items-center gap-1.5 rounded-full border border-gray-200/80 bg-white/70 px-3 py-1 text-xs font-medium backdrop-blur-sm transition-colors hover:border-primary-300 dark:border-gray-700/60 dark:bg-gray-900/60 dark:hover:border-primary-700"
        >
          <UIcon :name="s.icon" class="size-3.5 shrink-0" :class="s.color" />
          <span class="tabular-nums text-gray-800 dark:text-gray-200">{{
            s.value
          }}</span>
          <span class="text-gray-400 dark:text-gray-500">{{ s.label }}</span>
        </NuxtLink>
      </div>
    </div>

    <!-- ── Active Todos ───────────────────────────────────────── -->
    <div
      class="mt-5 overflow-hidden rounded-2xl border border-gray-200 bg-white dark:border-white/15 dark:bg-gray-800/60"
    >
      <div
        class="flex items-center gap-3 border-b border-gray-100 px-4 py-3.5 dark:border-white/10"
      >
        <!-- Progress ring -->
        <div class="relative size-13 shrink-0">
          <svg class="size-13 -rotate-90" viewBox="0 0 68 68">
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
            {{ todoProgress }}%
          </span>
        </div>

        <div class="min-w-0 flex-1">
          <h2 class="text-sm font-semibold text-gray-800 dark:text-gray-300/70">
            Active Todos
          </h2>
          <p class="mt-0.5 text-xs text-gray-400">
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

      <!-- Loading -->
      <div
        v-if="todoStore.loading"
        class="flex items-center gap-2 px-4 py-6 text-sm text-gray-400"
      >
        <UIcon name="heroicons:arrow-path" class="size-4 animate-spin" />
        Loading…
      </div>

      <!-- Empty -->
      <div
        v-else-if="activeTodos.length === 0"
        class="flex flex-col items-center justify-center py-10 text-center"
      >
        <div
          class="flex items-center justify-center rounded-full bg-gray-100 p-2 dark:bg-gray-800"
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

      <!-- Todo list -->
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
            <p v-if="todo.dueDate" class="mt-0.5 text-xs text-gray-400">
              Due {{ formatDate(todo.dueDate) }}
            </p>
          </div>
          <span
            class="shrink-0 text-xs capitalize text-gray-400 dark:text-gray-500"
          >
            {{ todo.priority }}
          </span>
        </div>
      </div>
    </div>

    <!-- ── Recent Notes ───────────────────────────────────────── -->
    <div
      class="mt-4 overflow-hidden rounded-2xl border border-gray-200 bg-white dark:border-white/15 dark:bg-gray-800/60"
    >
      <div
        class="flex items-center justify-between border-b border-gray-100 px-4 py-3.5 dark:border-white/10"
      >
        <h2
          class="flex items-center gap-1.5 text-sm font-semibold text-gray-700 dark:text-gray-300/70"
        >
          <UIcon name="heroicons:document-text" class="size-4 text-violet-400" />
          Recent notes
        </h2>
        <NuxtLink
          to="/notes"
          class="text-xs font-medium text-primary-500 transition-colors hover:text-primary-600"
        >
          See all
        </NuxtLink>
      </div>

      <!-- Loading -->
      <div
        v-if="noteStore.loading"
        class="flex items-center gap-2 p-4 text-xs text-gray-400"
      >
        <UIcon name="heroicons:arrow-path" class="size-3.5 animate-spin" />
        Loading…
      </div>

      <!-- Empty -->
      <div
        v-else-if="recentNotes.length === 0"
        class="flex flex-col items-center justify-center py-10 text-center"
      >
        <div
          class="flex items-center justify-center rounded-full bg-gray-100 p-2 dark:bg-gray-800"
        >
          <UIcon
            name="heroicons:document-text"
            class="size-6 text-gray-400 dark:text-gray-500"
          />
        </div>
        <p class="mt-3 text-xs font-medium text-gray-600 dark:text-gray-400">
          No notes yet
        </p>
        <p class="mt-0.5 text-xs text-gray-400 dark:text-gray-500">
          Your recent notes will appear here.
        </p>
      </div>

      <!-- Notes list -->
      <div
        v-else
        class="divide-y divide-gray-100 dark:divide-gray-700/60"
      >
        <NuxtLink
          v-for="note in recentNotes"
          :key="note.identifier"
          to="/notes"
          class="flex items-start gap-2.5 px-4 py-3 transition-colors"
        >
          <div
            class="mt-0.5 flex size-6 shrink-0 items-center justify-center rounded-md bg-violet-50 dark:bg-violet-950/60"
          >
            <UIcon
              name="heroicons:document-text"
              class="size-3 text-violet-400"
            />
          </div>
          <div class="min-w-0 flex-1">
            <p class="truncate text-xs font-medium text-gray-800 dark:text-gray-200">
              {{ note.title }}
            </p>
            <p class="mt-0.5 truncate text-xs text-gray-400">
              {{ stripHtml(note.content) || "No content" }}
            </p>
          </div>
          <span class="shrink-0 text-xs text-gray-300 dark:text-gray-600">
            {{ formatDate(note.updatedAt) }}
          </span>
        </NuxtLink>
      </div>
    </div>

    <!-- ── Recent Bookmarks ───────────────────────────────────── -->
    <div
      class="mt-4 overflow-hidden rounded-2xl border border-gray-200 bg-white dark:border-white/15 dark:bg-gray-800/60"
    >
      <div
        class="flex items-center justify-between border-b border-gray-100 px-4 py-3.5 dark:border-white/10"
      >
        <h2
          class="flex items-center gap-1.5 text-sm font-semibold text-gray-700 dark:text-gray-300/70"
        >
          <UIcon name="heroicons:bookmark" class="size-4 text-primary-400" />
          Recent bookmarks
        </h2>
        <NuxtLink
          to="/bookmarks"
          class="text-xs font-medium text-primary-500 transition-colors hover:text-primary-600"
        >
          See all
        </NuxtLink>
      </div>

      <!-- Loading -->
      <div
        v-if="bookmarkStore.loading"
        class="flex items-center gap-2 p-4 text-xs text-gray-400"
      >
        <UIcon name="heroicons:arrow-path" class="size-3.5 animate-spin" />
        Loading…
      </div>

      <!-- Empty -->
      <div
        v-else-if="recentBookmarks.length === 0"
        class="flex flex-col items-center justify-center py-10 text-center"
      >
        <div
          class="flex items-center justify-center rounded-full bg-gray-100 p-2 dark:bg-gray-800"
        >
          <UIcon
            name="heroicons:bookmark"
            class="size-6 text-gray-400 dark:text-gray-500"
          />
        </div>
        <p class="mt-3 text-xs font-medium text-gray-600 dark:text-gray-400">
          No bookmarks yet
        </p>
        <p class="mt-0.5 text-xs text-gray-400 dark:text-gray-500">
          Saved bookmarks will appear here.
        </p>
      </div>

      <!-- Bookmarks list -->
      <div
        v-else
        class="divide-y divide-gray-100 dark:divide-gray-700/60"
      >
        <div
          v-for="bm in recentBookmarks"
          :key="bm.identifier"
          class="flex items-start gap-3 px-4 py-3 transition-colors"
          @click="openUrl(bm.url)"
        >
          <div
            class="mt-0.5 flex size-7 shrink-0 items-center justify-center rounded-lg bg-primary-50 dark:bg-primary-950/60"
          >
            <UIcon
              name="heroicons:bookmark-solid"
              class="size-3.5 text-primary-400"
            />
          </div>
          <div class="min-w-0 flex-1">
            <p
              class="truncate text-xs font-medium text-gray-800 dark:text-gray-200"
            >
              {{ bm.title }}
            </p>
            <p class="mt-0.5 truncate text-xs text-gray-400">
              {{ bm.url }}
            </p>
            <span
              v-if="bm.tag"
              class="mt-1.5 inline-block rounded bg-gray-100 px-1.5 py-0.5 text-xs capitalize text-gray-500 dark:bg-gray-800 dark:text-gray-400"
              >{{ bm.tag }}</span
            >
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
