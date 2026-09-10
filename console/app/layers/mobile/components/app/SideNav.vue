<script setup lang="ts">
import { useUserPreferenceStore } from "@shared/stores/workspace-profile";

const preferenceStore = useUserPreferenceStore();

const { mobileNavOpen } = useMobileNav();
</script>

<template>
  <USlideover
    v-model:open="mobileNavOpen"
    side="left"
    :ui="{ content: 'max-w-64' }"
  >
    <template #content>
      <div class="flex flex-col h-full bg-white dark:bg-app-dark-800">
        <!-- Safe-area spacer -->
        <div class="shrink-0" style="height: env(safe-area-inset-top)" />

        <!-- Header -->
        <div
          class="flex items-center justify-between px-4 py-4 border-b border-gray-200 dark:border-gray-800 shrink-0"
        >
          <UUser
            :name="preferenceStore.fullName || 'Lunar User'"
            :description="preferenceStore.preference?.email"
            :avatar="{ icon: 'i-lucide-user' }"
            class="min-w-0 flex-1 truncate"
          />
          <UButton
            size="sm"
            color="neutral"
            variant="ghost"
            icon="heroicons:x-mark"
            aria-label="Close menu"
            @click="mobileNavOpen = false"
          />
        </div>

        <NavigationSideNavContent @navigate="mobileNavOpen = false" />

        <!-- Safe-area spacer -->
        <div class="shrink-0" style="height: env(safe-area-inset-bottom)" />
      </div>
    </template>
  </USlideover>
</template>