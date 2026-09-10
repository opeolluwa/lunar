<script lang="ts" setup>
import { primaryRoutes, secondaryRoutes } from "@shared/data/routes";
import { kNavbar, kPage } from "konsta/vue";
import _ from "lodash";
const route = useRoute();
const router = useRouter();
const { toggleMobileNav } = useMobileNav();

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

</script>

<template>
  <kPage class="h-dvh overflow-hidden flex flex-col">
    <kNavbar
      :title="isTopLevel ? undefined : pageTitle"
      :center-title="false"
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
          @click="toggleMobileNav"
        />

        <button v-else class="inline-flex items-center" @click="router.back()">
          <UIcon name="lucide:arrow-left" class="size-5" />
        </button>
      </template>

      <template #right>
      <AppHeaderRightPartial/>
      </template>
    </kNavbar>

    <main class="flex min-h-0 flex-1 flex-col overflow-hidden">
      <AppViewport class="bg-white/90 dark:bg-app-dark-800">
          
        <slot />
      </AppViewport>
      <AppBottonNav />
       <AppSideNav /> 
    </main>
  </kPage>
</template>
