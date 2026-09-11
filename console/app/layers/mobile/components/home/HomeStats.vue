<script setup lang="ts">
import { useNoteStore } from "@shared/stores/notes";
import { useBookmarkStore } from "@shared/stores/bookmarks";
import { useTodoStore } from "@shared/stores/todo";

const noteStore = useNoteStore();
const bookmarkStore = useBookmarkStore();
const todoStore = useTodoStore();

const statPills = computed(() => [
  {
    // label: "Notes",
    value: noteStore.notes.length,
    icon: "heroicons:document-text-solid",
    href: "/notes",
  },
  {
    // label: "Bookmarks",
    value: bookmarkStore.bookmarks.length,
    icon: "heroicons:bookmark-solid",
    href: "/bookmarks",
  },
  {
    // label: "Active",
    value: todoStore.activeTodos.length,
    icon: "heroicons:check-circle-solid",
    href: "/todo",
  },
]);
</script>

<template>
  <div class="mt-4 flex gap-2.5">
    <NuxtLink
      v-for="s in statPills"
      :key="s.label"
      :to="s.href"
      class="flex min-w-0 flex-1 items-center gap-2.5 rounded-xl border border-gray-200/80 bg-white px-3.5 py-2.5 backdrop-blur-sm transition-colors active:scale-[0.97] dark:border-gray-700/60 dark:bg-gray-900/60"
    >
      <div
        class="flex size-8 shrink-0 items-center justify-center rounded-lg bg-primary-50 dark:bg-primary-950/40"
      >
        <UIcon :name="s.icon" class="size-4 text-primary-500" />
      </div>
      <div class="min-w-0">
        <p
          class="text-base font-bold tabular-nums text-gray-800 dark:text-gray-200"
        >
          {{ s.value }}
        </p>
        <p class="text-[11px] text-gray-400 dark:text-gray-500">
          {{ s.label }}
        </p>
      </div>
    </NuxtLink>
  </div>
</template>
