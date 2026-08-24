<script lang="ts" setup>
import { primaryRoutes, secondaryRoutes } from "@shared/data/routes";
import { kPage, kNavbar, kNavbarBackLink } from "konsta/vue";

const route = useRoute();
const router = useRouter();
const { mobileNavOpen, toggleMobileNav } = useMobileNav();

const topLevelPaths = [...primaryRoutes, ...secondaryRoutes].map(
  (item) => item.path,
);
const isTopLevel = computed(() => topLevelPaths.includes(route.path));

const showEditorToolBar = computed(() => {
  return (
    route.path.includes("/create-notes") || route.path.includes("/edit-notes")
  );
});
const editorHeaderTitle = computed(() =>
  route.path.includes("/create-notes") ? "New note" : "Edit notes",
);
</script>

<template>
  <kPage>
    <main
      id="default_layout_mobile"
      class="flex h-dvh flex-col overflow-hidden bg-gray-50 dark:bg-app-dark-800"
    >
      <kNavbar
        v-if="!showEditorToolBar"
        bg-class="bg-white dark:bg-app-dark-800"
        class="absolute inset-x-0 top-0 z-40 flex max-h-20 items-center justify-between px-6 border-gray-200 dark:border-gray-800 dark:text-gray-500"
      >
        <template #left>
          <UButton
            v-if="isTopLevel"
            size="md"
            color="neutral"
            variant="ghost"
            icon="heroicons:bars-3"
            class="text-gray-400 dark:text-gray-500"
            aria-label="Open menu"
            @click="toggleMobileNav()"
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
          <div class="flex items-center gap-1">
            <UButton
              size="md"
              class="text-gray-400 dark:text-gray-500"
              color="neutral"
              variant="ghost"
              icon="heroicons:bell"
              aria-label="Notifications"
              @click="navigateTo('/notifications')"
            />
          </div>
        </template>
      </kNavbar>

      <kNavbar
        v-else
        :center-title="false"
        title-class="truncate text-sm font-medium"
        bg-class="bg-white dark:bg-app-dark-800"
        class="absolute inset-x-0 top-0 z-40 flex max-h-20 items-center justify-between px-6 border-gray-200 dark:border-gray-800 dark:text-gray-500"
      >
        <template #left>
          <NuxtLink class="inline-flex" @click="router.back()">
            <UIcon name="lucide:arrow-left" class="size-5" />
          </NuxtLink>
        </template>

        <template #title>{{ editorHeaderTitle }}</template>
      </kNavbar>

      <AppViewport :hide-header-and-nav="showEditorToolBar">
        <slot />
      </AppViewport>

      <AppBottonNav v-if="!showEditorToolBar" />
      <AppSideNav :mobile-nav-open="mobileNavOpen" />
    </main>
  </kPage>
</template>
