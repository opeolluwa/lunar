<script setup lang="ts">
import { primaryRoutes, secondaryRoutes } from "@shared/data/routes";
import { useAuthStore } from "@shared/stores/auth";
import { useUserPreferenceStore } from "@shared/stores/workspace-profile";

const route = useRoute();
const colorMode = useColorMode();

const {
  mobileNavOpen,
  closeMobileNav,
} = useMobileNav();

const authStore = useAuthStore();
const preferenceStore = useUserPreferenceStore();

const isDark = computed({
  get: () => colorMode.value === "dark",
  set: (value) => {
    colorMode.preference = value ? "dark" : "light";
  },
});

const themeIcon = computed(() =>
  isDark.value
    ? "heroicons:sun"
    : "heroicons:moon",
);

const themeLabel = computed(() =>
  isDark.value
    ? "Light mode"
    : "Dark mode",
);

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

  return (
    route.path === path ||
    route.path.startsWith(`${path}/`)
  );
}

function navigate(path: string) {
  closeMobileNav();
  navigateTo(path);
}
</script>

<template>
  <USlideover
    v-model:open="mobileNavOpen"
    side="left"
    :ui="{ content: 'max-w-64' }"
  >
    <template #content>
      <div
        class="flex h-full flex-col bg-white dark:bg-app-dark-900"
      >
        <!-- Safe area -->
        <div
          class="shrink-0"
          style="height: env(safe-area-inset-top)"
        />

        <!-- User header -->
        <div
          class="flex shrink-0 items-center justify-between border-b border-gray-200 px-4 py-4 dark:border-gray-800"
        >
          <UUser
            :name="preferenceStore.fullName"
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
            @click="closeMobileNav"
          />
        </div>

        <!-- Primary navigation -->
        <nav
          class="scrollbar-config flex flex-1 flex-col gap-0.5 overflow-y-auto px-2 py-2"
        >
          <NuxtLink
            v-for="item in primaryRoutes"
            :key="item.name"
            :to="item.path"
            class="flex cursor-pointer items-center gap-3 rounded-lg px-3 py-2 text-sm transition-colors"
            :class="
              isActive(item.path)
                ? 'bg-primary-50 font-medium text-primary-700 dark:bg-primary-950 dark:text-primary-300'
                : 'text-gray-600 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-800'
            "
            @click="closeMobileNav"
          >
            <UIcon
              :name="
                isActive(item.path)
                  ? item.activeIcon || item.icon
                  : item.icon
              "
              class="size-4 shrink-0"
            />

            <span class="truncate">
              {{ item.name }}
            </span>
          </NuxtLink>
        </nav>

        <!-- Bottom actions -->
        <div
          class="flex shrink-0 flex-col gap-0.5 px-2 pb-4"
        >
          <USeparator class="mx-1 mb-2" />

          <!-- Theme -->

          <!-- Secondary routes -->
        

          <!-- Logout -->
          <button
            type="button"
            class="flex w-full cursor-pointer items-center gap-3 rounded-lg px-3 py-2 text-sm text-red-500 transition-colors hover:bg-red-50 dark:text-red-400 dark:hover:bg-red-950/40"
            @click="logout"
          >
            <UIcon
              name="heroicons:arrow-right-start-on-rectangle"
              class="size-4 shrink-0"
            />

            Logout
          </button>
        </div>
      </div>
    </template>
  </USlideover>
</template>