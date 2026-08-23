<script setup lang="ts">
import {
  kTabbar,
  kTabbarLink,
  kToolbarPane,
} from "konsta/vue";

import { mobileBottomNavRoutes } from "@shared/data/routes";

const route = useRoute();

function isActive(path: string): boolean {
  if (path === "/") {
    return route.path === "/";
  }

  return route.path === path || route.path.startsWith(`${path}/`);
}

function navigate(path: string) {
  navigateTo(path);
}
</script>

<template>
  <kTabbar
    labels
    icons
    class="fixed bottom-0 left-0 z-50 w-full"
     bg-class="bg-white dark:bg-app-dark-800"
  >
    <kToolbarPane>
      <kTabbarLink
        v-for="item in mobileBottomNavRoutes"
        :key="item.path"
        :active="isActive(item.path)"
        @click="navigate(item.path)"
      >
        <template #label>
          {{ item.name }}
        </template>

        <template #icon>
          <UIcon
            :name="
              isActive(item.path)
                ? item.activeIcon || item.icon
                : item.icon
            "
            class="size-5"
          />
        </template>
      </kTabbarLink>
    </kToolbarPane>
  </kTabbar>
</template>