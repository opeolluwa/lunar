<script lang="ts" setup>
import { primaryRoutes, secondaryRoutes } from "@shared/data/routes";

import { kNavbar, kNavbarBackLink, kPage } from "konsta/vue";

const route = useRoute();
const router = useRouter();

const { toggleMobileNav } = useMobileNav();

const topLevelPaths = [...primaryRoutes, ...secondaryRoutes].map(
  (item) => item.path,
);

const isTopLevel = computed(() => topLevelPaths.includes(route.path));

const hideHeaderAndNav = computed(() => {
  return (
    route.path.includes("/create-notes") || route.path.includes("/edit-notes")
  );
});
</script>

<template>
  <kPage>
    <kNavbar bg-class="bg-white dark:bg-app-dark-800" class="px-3">
      <template #left>
        <UButton
          v-if="isTopLevel"
          size="md"
          color="neutral"
          variant="ghost"
          icon="heroicons:bars-3"
          class="text-gray-400 dark:text-gray-500"
          aria-label="Open menu"
          @click="toggleMobileNav"
        />

        <kNavbarBackLink
          v-else
          text="Back"
          component="div"
          class="size-5 text-gray-400 dark:text-gray-500"
          @click="router.back()"
        />
      </template>

      <template #right>
        <UButton
          size="md"
          color="neutral"
          variant="ghost"
          icon="heroicons:bell"
          class="text-gray-400 dark:text-gray-500"
          aria-label="Notifications"
          @click="navigateTo('/notifications')"
        />
      </template>
    </kNavbar>

    <main
      id="default_layout_mobile"
      class="flex h-dvh flex-col overflow-hidden bg-gray-50 dark:bg-app-dark-800"
    >
      <AppViewport :hide-header-and-nav="hideHeaderAndNav">
        <slot />
      </AppViewport>

      <AppBottonNav v-if="!hideHeaderAndNav" />

      <!-- DO NOT use v-if here -->
      <AppSideNav />
    </main>
  </kPage>
</template>
