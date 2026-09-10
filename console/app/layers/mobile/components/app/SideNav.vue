<script setup lang="ts">
import { primaryRoutes, secondaryRoutes } from "@shared/data/routes";
import { useAuthStore } from "@shared/stores/auth";
import { useUserPreferenceStore } from "@shared/stores/workspace-profile";
import NavigationApp from "@desktop/components/navigation/app.vue"
const route = useRoute();
const colorMode = useColorMode();

const { mobileNavOpen, closeMobileNav } = useMobileNav();

const authStore = useAuthStore();
const preferenceStore = useUserPreferenceStore();

function logout() {
  closeMobileNav();

  authStore.clearSession();
  authStore.exitGuestMode();

  navigateTo("/auth/login");
}

function isActive(path: string): boolean {
  if (path === "/") {
    return route.path === "/";
  }

  return route.path === path || route.path.startsWith(`${path}/`);
}
</script>

<template>
  <USlideover
    v-model:open="mobileNavOpen"
    side="left"
    :ui="{ content: 'max-w-64' }"
  >
    <template #content>
     <NavigationApp/>
    </template>
  </USlideover>
</template>
