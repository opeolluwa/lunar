<script lang="ts" setup>
import { primaryRoutes, secondaryRoutes } from "@shared/data/routes";
import { kPage, kNavbar } from "konsta/vue";

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

const pageTitle = computed(() => route.meta.name);
useHead({ title: () => pageTitle.value as string });
</script>

<template>
  <kPage>
    <main
      id="default_layout_mobile"
      class="flex h-dvh flex-col overflow-hidden"
    >
      <kNavbar
        v-if="!showEditorToolBar"
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
            @click="toggleMobileNav()"
          />

          <button
            v-else
            class="inline-flex items-center"
            @click="router.back()"
          >
            <UIcon name="lucide:arrow-left" class="size-5" />
          </button>
        </template>

        <template #right>
          <div class="flex items-center gap-1">
            <UButton
              size="md"
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
        class="shrink-0 px-2"
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
