<script setup lang="ts">
import { useBookmarkStore } from "@shared/stores/bookmarks";
import { safeOpenUrl as openUrl } from "@shared/utils/safe-open-url";
import AppFab from "@mobile/components/app/fab.vue";
import BookmarkCard from "@mobile/components/bookmark/bookmark-card.vue";
import EmptyState from "@shared/components/app/EmptyState.vue";

definePageMeta({ name: "Bookmarks" });

const bookmarkStore = useBookmarkStore();
const { notify } = useAppNotification();

const showCreatePopup = ref(false);

onMounted(() => {
  bookmarkStore.fetchBookmarks();
});

function handleCreated() {
  notify({ message: "Bookmark created", type: "success" });
}
</script>

<template>
  <AppPullToRefresh @refresh="() => bookmarkStore.fetchBookmarks()">
    <div>
      <!-- Create bookmark FAB -->
      <AppFab
        v-if="bookmarkStore.bookmarks.length !== 0"
        @click="showCreatePopup = true"
      />

      <!-- Loading -->
      <div v-if="bookmarkStore.loading" class="flex flex-col gap-3">
        <USkeleton v-for="i in 4" :key="i" class="h-24 rounded-lg" />
      </div>

      <!-- Empty state: no bookmarks at all -->
      <div v-else-if="bookmarkStore.bookmarks.length === 0">
        <EmptyState
          title="No bookmarks yet"
          description="Create your first bookmark to get started."
          icon="ri:bookmark-line"
          action-label="create bookmark"
          @action="showCreatePopup = true"
        />
      </div>

      <!-- Bookmark list -->
      <div v-else class="flex flex-col gap-3">
        <BookmarkCard
          v-for="bookmark in bookmarkStore.bookmarks"
          :key="bookmark.identifier"
          :bookmark="bookmark"
          @delete="(id) => bookmarkStore.deleteBookmark(id)"
          @preview="(bm) => openUrl(bm.url)"
        />
      </div>

      <!-- Create bookmark popup -->
      <BookmarkCreatePopup
        v-model:open="showCreatePopup"
        @created="handleCreated"
      />
    </div>
  </AppPullToRefresh>
</template>
