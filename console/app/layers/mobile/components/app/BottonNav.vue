<script setup lang="ts">
import { mobileBottomNavRoutes } from "@shared/data/routes";
import ToolBarWrapper from "./ToolBarWrapper.vue";

const route = useRoute();

function isActive(path: string): boolean {
  if (path === "/") return route.path === "/";
  return route.path.startsWith(path);
}
</script>

<template>
  <ToolBarWrapper>
    <NuxtLink
      v-for="item in mobileBottomNavRoutes"
      :key="item.path"
      :to="item.path"
      class="flex touch-ripple-red-500 flex-col items-center gap-0.5 py-2 px-3 text-[10px] transition-colors"
      :class="
        isActive(item.path)
          ? 'text-primary-500 dark:text-primary-400'
          : 'text-gray-400 dark:text-gray-500'
      "
    >
      <UIcon
        :name="isActive(item.path) ? item.activeIcon : item.icon"
        class="size-5"
      />
      {{ item.name }}
    </NuxtLink>
  </ToolBarWrapper>
</template>
