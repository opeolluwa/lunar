<script setup lang="ts">
import type { BookmarkInterface } from "lunar";
import { safeOpenUrl as openUrl } from "@shared/utils/safe-open-url";

defineProps<{
  bookmarks: BookmarkInterface[];
  loading: boolean;
}>();
</script>

<template>
  <div
    class="bg-white dark:bg-gray-800/60 rounded-2xl border border-gray-200 dark:border-white/15 overflow-hidden"
  >
    <div
      class="flex items-center justify-between px-5 py-3.5 border-b border-gray-100 dark:border-white/10"
    >
      <h2
        class="text-sm font-semibold text-gray-700 dark:text-gray-300/70 flex items-center gap-1.5"
      >
        <UIcon name="heroicons:bookmark" class="size-4 text-primary-400" />
        Recent bookmarks
      </h2>
    </div>

    <div
      v-if="loading"
      class="flex items-center gap-2 p-5 text-gray-400 text-sm"
    >
      <UIcon name="heroicons:arrow-path" class="size-4 animate-spin" />
      Loading…
    </div>

    <div
      v-else-if="bookmarks.length === 0"
      class="flex flex-col items-center justify-center py-10 text-center"
    >
      <div
        class="p-2 flex justify-center items-center rounded-full bg-gray-100 dark:bg-gray-800"
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

    <div
      v-else
      class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 divide-y sm:divide-y-0 sm:divide-x divide-gray-100 dark:divide-gray-700/60"
    >
      <div
        v-for="bm in bookmarks"
        :key="bm.identifier"
        class="group flex items-start gap-3 p-4 hover:bg-gray-50 dark:hover:bg-gray-800/40 transition-colors cursor-pointer"
        @click="openUrl(bm.url)"
      >
        <div
          class="size-7 rounded-lg bg-primary-50 dark:bg-primary-950/60 flex items-center justify-center shrink-0 mt-0.5"
        >
          <UIcon
            name="heroicons:bookmark-solid"
            class="size-3.5 text-primary-400"
          />
        </div>
        <div class="flex-1 min-w-0">
          <p
            class="text-xs font-medium text-gray-800 dark:text-gray-200 truncate group-hover:text-primary-600 dark:group-hover:text-primary-400 transition-colors"
          >
            {{ bm.title }}
          </p>
          <p class="text-xs text-gray-400 truncate mt-0.5">
            {{ bm.url }}
          </p>
          <span
            v-if="bm.tag"
            class="inline-block mt-1.5 text-xs px-1.5 py-0.5 rounded bg-gray-100 dark:bg-gray-800 text-gray-500 dark:text-gray-400 capitalize"
            >{{ bm.tag }}</span
          >
        </div>
      </div>
    </div>
  </div>
</template>
