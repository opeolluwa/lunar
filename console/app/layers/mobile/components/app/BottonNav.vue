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

  return (
    route.path === path ||
    route.path.startsWith(`${path}/`)
  );
}

function navigate(path: string) {
  navigateTo(path);
}
</script>

<template>
  <div class="fixed inset-x-0 bottom-0 z-50">
    <kTabbar
      labels
      icons
      class="[&_.k-link]:w-auto [&_.k-link]:min-w-0 [&_.k-link]:flex-1 [&_.k-link>span]:gap-0 [&_.k-tabbar-link-icon]:h-7 material:[&_.k-tabbar-link-icon]:w-12"
      bg-class="bg-white dark:bg-app-dark-800"
    >
      <kToolbarPane>
        <kTabbarLink
          v-for="item in mobileBottomNavRoutes"
          :key="item.path"
          :active="isActive(item.path)"
          :colors="
            isActive(item.path)
              ? {
                  textActiveIos: 'text-primary-500 dark:text-primary-400',
                  textActiveMaterial: 'text-primary-500 dark:text-primary-400',
                }
              : {}
          "
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
  </div>
</template>