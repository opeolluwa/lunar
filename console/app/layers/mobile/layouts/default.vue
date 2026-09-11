<script lang="ts" setup>
import { primaryRoutes, secondaryRoutes } from "@shared/data/routes";
import { kNavbar, kPage } from "konsta/vue";
import _ from "lodash";
const route = useRoute();
const router = useRouter();
const { toggleMobileNav } = useMobileNav();
const { isActive, searchQuery, closeSearch } = useMobileSearch();

const searchInputRef = useTemplateRef("searchInputRef");

const topLevelPaths = [...primaryRoutes, ...secondaryRoutes].map(
  (item) => item.path,
);
const isTopLevel = computed(() => topLevelPaths.includes(route.path));
const pageTitle = computed(() => {
  const raw = route.name?.toString().replaceAll("-", " ") ?? "";
  return raw
    .split(" ")
    .map((w) => _.capitalize(w))
    .join(" ");
});

watch(isActive, (active) => {
  if (active) {
    nextTick(() => searchInputRef.value?.inputRef?.focus());
  } else {
    searchInputRef.value?.inputRef?.blur();
  }
});

watch(
  () => route.fullPath,
  () => {
    if (isActive.value) closeSearch();
  },
);
</script>

<template>
  <kPage class="h-dvh overflow-hidden flex flex-col">
    <kNavbar
      :title="!isActive && !isTopLevel ? pageTitle : undefined"
      :center-title="false"
      :title-class="
        isActive ? 'flex-1 min-w-0' : 'truncate text-md pl-4 font-medium'
      "
      bg-class="bg-white dark:bg-app-dark-800"
      class="shrink-0 px-2"
    >
      <template #left>
        <UButton
          v-if="isTopLevel"
          size="md"
          color="neutral"
          variant="ghost"
          icon="heroicons:bars-3"
          aria-label="Open menu"
          @click="toggleMobileNav"
        />

        <button v-else class="inline-flex items-center" @click="router.back()">
          <UIcon name="lucide:arrow-left" class="size-5" />
        </button>
      </template>

      <template #title>
        <div v-if="isActive" class="flex w-full min-w-0 items-center gap-2">
          <UInput
            ref="searchInputRef"
            v-model="searchQuery"
            size="md"
            placeholder="Search..."
            variant="none"
            autocapitalize="off"
            autocorrect="off"
            spellcheck="false"
            :ui="{ trailing: 'pe-1', root: 'ml-4 border-none' }"
            @keydown.escape="closeSearch"
          >
            <template #trailing>
              <UButton
                color="neutral"
                variant="link"
                size="sm"
                icon="i-lucide-circle-x"
                :aria-label="searchQuery ? 'Clear input' : 'Close search'"
                @click="searchQuery ? (searchQuery = '') : closeSearch()"
              />
            </template>
          </UInput>
        </div>
      </template>

      <template #right>
        <AppHeaderRightPartial v-if="!isActive" />
      </template>
    </kNavbar>

    <main v-if="!isActive" class="flex min-h-0 flex-1 flex-col overflow-hidden">
      <AppViewport class="bg-white/90 dark:bg-app-dark-800">
        <slot />
      </AppViewport>
      <AppBottonNav />
      <AppSideNav />
    </main>

    <AppGlobalSearch v-else />
  </kPage>
</template>
