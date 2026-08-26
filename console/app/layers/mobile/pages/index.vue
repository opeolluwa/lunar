<script setup lang="ts">
import { useNoteStore } from "@shared/stores/notes";
import { useBookmarkStore } from "@shared/stores/bookmarks";
import { useTodoStore } from "@shared/stores/todo";
import { useUserPreferenceStore } from "@shared/stores/workspace-preferences";
import { kPreloader } from "konsta/vue";

import HomeHeader from "@mobile/components/home/HomeHeader.vue";
import HomeQuickActions from "@mobile/components/home/HomeQuickActions.vue";
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

const { container, isRefreshing, pullDistance } = usePullToRefresh({
  onRefresh: refreshAll,
  threshold: 64,
});
</script>

<template>
  <div ref="container">
    <Transition name="slide-down">
      <div
        v-if="isRefreshing || pullDistance > 0"
        class="flex justify-center py-2"
        :style="{ height: `${pullDistance}px` }"
      >
        <kPreloader
          :size="pullDistance >= 64 && isRefreshing ? 'w-6 h-6' : 'w-5 h-5'"
          class="text-primary-500"
        />
      </div>
    </Transition>

    <HomeHeader />
    <!-- <HomeQuickActions /> -->
    <HomeStats />
    <HomeTodos />
    <HomeNotes />
    <HomeBookmarks />
  </div>
</template>

<style scoped>
.slide-down-enter-active,
.slide-down-leave-active {
  transition: all 0.2s ease-out;
}
.slide-down-enter-from,
.slide-down-leave-to {
  opacity: 0;
  transform: translateY(-8px);
}
</style>
