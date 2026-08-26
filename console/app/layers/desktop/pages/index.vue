<script setup lang="ts">
import { useNoteStore } from "@shared/stores/notes";
import { useBookmarkStore } from "@shared/stores/bookmarks";
import { useTodoStore } from "@shared/stores/todo";
import { useUserPreferenceStore } from "@shared/stores/workspace-preferences";

definePageMeta({ layout: false });

const noteStore = useNoteStore();
const bookmarkStore = useBookmarkStore();
const todoStore = useTodoStore();
const userPreferenceStore = useUserPreferenceStore();

const { setSearch, clearSearch } = useAppSearch();

onMounted(async () => {
  await Promise.all([
    noteStore.fetchNotes(),
    bookmarkStore.fetchBookmarks(),
    todoStore.fetchTodos(),
    userPreferenceStore.fetchPreference(),
  ]);
  setSearch({ placeholder: "Search everything…" });
});

onUnmounted(() => clearSearch());

const userName = computed(
  () => userPreferenceStore.preference?.firstName || "there",
);

const recentNotes = computed(() => noteStore.notes.slice(0, 4));
const recentBookmarks = computed(() => bookmarkStore.bookmarks.slice(0, 4));

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
  <NuxtLayout name="default">
    <template #page_title>
      <HomeHeader
        :user-name="userName"
        :active-todo-count="todoStore.activeTodos.length"
        :stats="statPills"
      />
    </template>

    <template #main_content>
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-4">
        <div class="lg:col-span-2">
          <HomeTodos
            :todos="todoStore.todos"
            :loading="todoStore.loading"
            @delete="(id) => todoStore.deleteTodo(id)"
          />
        </div>
        <div>
          <HomeNotes :notes="recentNotes" :loading="noteStore.loading" />
        </div>
        <div class="lg:col-span-3">
          <HomeBookmarks
            :bookmarks="recentBookmarks"
            :loading="bookmarkStore.loading"
          />
        </div>
      </div>
    </template>
  </NuxtLayout>
</template>
