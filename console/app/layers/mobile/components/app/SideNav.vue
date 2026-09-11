<script setup lang="ts">
import { useUserPreferenceStore } from "@shared/stores/workspace-profile";
import { useAuthStore } from "@shared/stores/auth";

const preferenceStore = useUserPreferenceStore();
const authStore = useAuthStore();

const { mobileNavOpen } = useMobileNav();

function handleLogout() {
  authStore.clearSession();
  authStore.exitGuestMode();
  mobileNavOpen.value = false;
  navigateTo("/auth/login");
}
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

        <!-- Footer: logout -->
        <div class="shrink-0 px-3 pt-1 pb-4">
          <USeparator class="mb-3" />
          <button
            type="button"
            class="flex items-center w-full h-11 gap-2 px-2 rounded-xl text-red-500 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-500/10 hover:text-red-600 dark:hover:text-red-300 transition-colors"
            @click="handleLogout"
          >
            <UIcon
              name="heroicons:arrow-right-start-on-rectangle"
              class="size-5 shrink-0"
            />
            <span class="text-sm font-medium">
              {{ authStore.isGuest ? "Exit" : "Logout" }}
            </span>
          </button>
        </div>

        <!-- Safe-area spacer -->
        <div class="shrink-0" style="height: env(safe-area-inset-bottom)" />
      </div>
    </template>
  </USlideover>
</template>