<script setup lang="ts">
import { kFab } from "konsta/vue";
import { useBookmarkStore } from "@shared/stores/bookmarks";
import EmptyState from "@shared/components/app/EmptyState.vue";

definePageMeta({ name: "Bookmarks" });

const bookmarkStore = useBookmarkStore();
const { notify } = useAppNotification();

const showCreatePopup = ref(false);

const fabColors = {
  bgIos: "bg-primary-500 dark:bg-primary-600",
  bgMaterial: "bg-primary-500 dark:bg-primary-600",
  textIos: "text-white",
  textMaterial: "text-white",
};

function handleCreated() {
  notify({ message: "Bookmark created", type: "success" });
}
</script>

<template>
  <div>
    <!-- Create bookmark FAB -->
    <div
      v-if="bookmarkStore.bookmarks.length !== 0"
      class="fixed bottom-20 right-5 z-40"
    >
      <kFab
        component="button"
        aria-label="Add bookmark"
        :colors="fabColors"
        @click="showCreatePopup = true"
      >
        <template #icon>
          <UIcon name="heroicons:plus" class="size-6" />
        </template>
      </kFab>
    </div>

    <!-- Loading -->
    <div v-if="bookmarkStore.loading" class="flex flex-col gap-3">
      <USkeleton v-for="i in 4" :key="i" class="h-24 rounded-lg" />
    </div>

    <!-- Empty state: no bookmarks at all -->
    <div
      v-else-if="bookmarkStore.bookmarks.length === 0"
      class="flex flex-col items-center justify-center py-20 text-center"
    >
      <EmptyState
        title="No bookmarks yet"
        description="Create your first bookmark to get started."
        icon="ri:bookmark-line"
        action-label="create bookmark"
        @action="showCreatePopup = true"
      />
    </div>

    <!-- Bookmark list -->
    <!-- <div v-else class="flex flex-col gap-3">
      <BookmarkCard
        v-for="bookmark in filtered"
        :key="bookmark.identifier"
        :bookmark="bookmark"
        @delete="bookmarkStore.deleteBookmark"
        @preview="openUrl(bookmark.url)"
      />
    </div> -->

    <!-- Create bookmark popup -->
    <BookmarkCreatePopup
      v-model:open="showCreatePopup"
      @created="handleCreated"
    />
  </div>
</template>
