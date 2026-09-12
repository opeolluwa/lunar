<script setup lang="ts">
import { useSidebarStore } from "@shared/stores/sidebar";

const sidebarStore = useSidebarStore();
</script>

<template>
  <UDashboardSidebar
    id="app"
    v-model:collapsed="sidebarStore.collapsed"
    class="hidden md:flex"
    :collapsible="true"
    :collapsed-size="4"
    :default-size="18"
    :min-size="4"
    :max-size="42"
    :ui="{
      root: [
        'bg-white dark:bg-gray-950',
        'border-e border-gray-200 dark:border-gray-800',
        'overflow-hidden',
      ],
      header: 'shrink-0 h-auto p-0',
      body: 'flex flex-col flex-1 min-h-0 p-0',
      footer: 'shrink-0 p-0',
      handle: 'cursor-ew-resize',
    }"
  >
    <NavigationSideNavContent :collapsed="sidebarStore.collapsed" />
  </UDashboardSidebar>
</template>
