<script setup lang="ts">
import { useNoteStore } from "@shared/stores/notes";
import { useBookmarkStore } from "@shared/stores/bookmarks";
import { useTodoStore } from "@shared/stores/todo";

const noteStore = useNoteStore();
const bookmarkStore = useBookmarkStore();
const todoStore = useTodoStore();

const statPills = computed(() => [
  {
    label: "Notes",
    value: noteStore.notes.length,
    icon: "heroicons:document-text-solid",
    color: "text-violet-500",
    bg: "bg-violet-50 dark:bg-violet-950/40",
    href: "/notes",
  },
  {
    label: "Bookmarks",
    value: bookmarkStore.bookmarks.length,
    icon: "heroicons:bookmark-solid",
    color: "text-primary-500",
    bg: "bg-primary-50 dark:bg-primary-950/40",
    href: "/bookmarks",
  },
  {
    label: "Active",
    value: todoStore.activeTodos.length,
    icon: "heroicons:check-circle-solid",
    color: "text-emerald-500",
    bg: "bg-emerald-50 dark:bg-emerald-950/40",
    href: "/todo",
  },
]);
</script>

<template>
  <div
    class="no-scrollbar -mx-6 mt-4 flex snap-x snap-mandatory gap-2.5 overflow-x-auto px-6"
  >
    <NuxtLink
      v-for="s in statPills"
      :key="s.label"
      :to="s.href"
      class="flex min-w-[110px] flex-1 snap-start items-center gap-2.5 rounded-xl border border-gray-200/80 bg-white/70 px-3.5 py-2.5 backdrop-blur-sm transition-colors active:scale-[0.97] dark:border-gray-700/60 dark:bg-gray-900/60"
    >
      <div
        class="flex size-8 shrink-0 items-center justify-center rounded-lg"
        :class="s.bg"
      >
        <UIcon :name="s.icon" class="size-4" :class="s.color" />
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
