<script setup lang="ts">
import { primaryRoutes, secondaryRoutes } from "@shared/data/routes";
import { kNavbar, kNavbarBackLink } from "konsta/vue";

const route = useRoute();
const router = useRouter();
const { toggleMobileNav } = useMobileNav();

const topLevelPaths = [...primaryRoutes, ...secondaryRoutes].map(
  (routeItem) => routeItem.path,
);
const isTopLevel = computed(() => topLevelPaths.includes(route.path));
</script>

<template>
  <kNavbar
  
    bg-class="bg-white dark:bg-app-dark-800"
    class="absolute top-0 inset-x-0 max-h-20 flex items-center justify-between  dark:text-gray-500 border-gray-200 dark:border-gray-800"
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
        class="text-gray-400 dark:text-gray-500 size-5"
        @click="() => router.back()"
        component="div"
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
</template>
