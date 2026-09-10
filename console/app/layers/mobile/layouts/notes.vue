<script lang="ts" setup>
import { primaryRoutes, secondaryRoutes } from "@shared/data/routes";
import { kPage, kNavbar } from "konsta/vue";
import _ from "lodash";

const route = useRoute();
const router = useRouter();
const { toggleMobileNav } = useMobileNav();
const pageTitle = computed(() => {
  const raw = route.name?.toString().replaceAll("-", " ") ?? "";
  return raw
    .split(" ")
    .map((w) => _.capitalize(w))
    .join(" ");
});

useKeyboardInset();

const topLevelPaths = [...primaryRoutes, ...secondaryRoutes].map(
  (item) => item.path,
);
const isTopLevel = computed(() => topLevelPaths.includes(route.path));

const showEditorToolBar = computed(() => {
  return (
    route.path.includes("/create-notes") || route.path.includes("/edit-notes")
  );
});

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
        :title="isTopLevel ? undefined : pageTitle"
        title-class="truncate text-md pl-4 font-medium"
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
          <AppHeaderRightPartial />
        </template>
      </kNavbar>

      <kNavbar
        v-else
        :center-title="false"
        :title="isTopLevel ? undefined : pageTitle"
        title-class="truncate text-md pl-4 font-medium"
        bg-class="bg-white dark:bg-app-dark-800"
        class="shrink-0 px-2"
      >
        <template #left>
          <NuxtLink class="inline-flex" @click="router.back()">
            <UIcon name="lucide:arrow-left" class="size-5" />
          </NuxtLink>
        </template>
      </kNavbar>

      <AppViewport :hide-header-and-nav="showEditorToolBar">
        <slot />
      </AppViewport>

      <AppBottonNav v-if="!showEditorToolBar" />
      <AppSideNav />
    </main>
  </kPage>
</template>
