<script setup lang="ts">
import { useBookmarkStore } from "@shared/stores/bookmarks";
import { safeOpenUrl as openUrl } from "@shared/utils/safe-open-url";

const bookmarkStore = useBookmarkStore();

const recentBookmarks = computed(() => bookmarkStore.bookmarks.slice(0, 3));
</script>

<template>
  <div
    class="mt-4 overflow-hidden rounded-2xl border border-gray-200 bg-white dark:border-white/15 dark:bg-gray-800/60"
  >
    <div
      class="flex items-center justify-between border-b border-gray-100 px-4 py-3 dark:border-white/10"
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

    <div
      v-if="bookmarkStore.loading"
      class="flex items-center gap-2 p-4 text-xs text-gray-400"
    >
      <UIcon name="heroicons:arrow-path" class="size-3.5 animate-spin" />
      Loading…
    </div>

    <div
      v-else-if="recentBookmarks.length === 0"
      class="flex flex-col items-center justify-center py-8 text-center"
    >
      <div
        class="flex items-center justify-center rounded-full bg-gray-100 p-2 dark:bg-gray-800"
      >
        <UIcon
          name="heroicons:bookmark"
          class="size-5 text-gray-400 dark:text-gray-500"
        />
      </div>
      <p class="mt-2 text-xs font-medium text-gray-600 dark:text-gray-400">
        No bookmarks yet
      </p>
      <p class="mt-0.5 text-[11px] text-gray-400 dark:text-gray-500">
        Saved bookmarks will appear here.
      </p>
    </div>

    <div
      v-else
      class="divide-y divide-gray-100 dark:divide-gray-700/60"
    >
      <div
        v-for="bm in recentBookmarks"
        :key="bm.identifier"
        class="flex items-start gap-3 px-4 py-3 transition-colors active:bg-gray-50 dark:active:bg-white/5"
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
          <p class="mt-0.5 truncate text-[11px] text-gray-400">
            {{ bm.url }}
          </p>
          <span
            v-if="bm.tag"
            class="mt-1 inline-block rounded bg-gray-100 px-1.5 py-0.5 text-[10px] capitalize text-gray-500 dark:bg-gray-800 dark:text-gray-400"
            >{{ bm.tag }}</span
          >
        </div>
      </div>
    </div>
  </div>
</template>
