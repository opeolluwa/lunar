<script setup lang="ts">
import { useNoteStore } from "@shared/stores/notes";
import { useBookmarkStore } from "@shared/stores/bookmarks";
import { useTodoStore } from "@shared/stores/todo";
import { useUserPreferenceStore } from "@shared/stores/workspace-profile";

import HomeHeader from "@mobile/components/home/HomeHeader.vue";
import HomeStats from "@mobile/components/home/HomeStats.vue";
import HomeTodos from "@mobile/components/home/HomeTodos.vue";

const HomeNotes = defineAsyncComponent(
  () => import("@mobile/components/home/HomeNotes.vue"),
);
const HomeBookmarks = defineAsyncComponent(
  () => import("@mobile/components/home/HomeBookmarks.vue"),
);

definePageMeta({ layout: "default", name: "Home" });

const noteStore = useNoteStore();
const bookmarkStore = useBookmarkStore();
const todoStore = useTodoStore();
const userPreferenceStore = useUserPreferenceStore();

async function refreshAll() {
  await Promise.all([
    noteStore.fetchNotes(),
    bookmarkStore.fetchBookmarks(),
    todoStore.fetchTodos(),
    userPreferenceStore.fetchPreference(),
  ]);
}

onMounted(refreshAll);
</script>

<template>
  <PullToRefresh @refresh="refreshAll">
    <HomeHeader />
    <HomeStats />
    <HomeTodos />
    <HomeNotes />
    <HomeBookmarks />
  </PullToRefresh>
</template>
