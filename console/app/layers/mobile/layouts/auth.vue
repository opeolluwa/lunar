<script lang="ts" setup>
import { kNavbar, kNavbarBackLink, kPage } from "konsta/vue";

const route = useRoute();
const router = useRouter();

const kbInset = useKeyboardInset();

const isLoginPage = computed(() => route.path === "/auth/login");

const canGoBack = computed(() => window.history.length > 1);

const showBackButton = computed(() => !isLoginPage.value || canGoBack.value);

function goBack() {
  if (canGoBack.value) {
    router.back();
    return;
  }

  navigateTo("/auth/login");
}
</script>

<template>
  <div>
    <kNavbar
      transparent
      bg-class="bg-white dark:bg-app-dark-800"
      class="px-2 mt-0"
    >
      <template #left>
        <button class="inline-flex items-center" @click="router.back()">
          <UIcon name="lucide:arrow-left" class="size-5" />
        </button>
      </template>

      <template #right>
        <NuxtLink
          v-if="route.path === '/auth/welcome-back'"
          to="/auth/login"
          class="inline-flex"
        >
          <span
            class="whitespace-nowrap text-xs font-semibold text-primary-500 hover:text-primary-600"
          >
            Not Adeoye?
          </span>
        </NuxtLink>
      </template>
    </kNavbar>

    <main
      id="auth_layout_mobile"
      class="flex h-dvh flex-col overflow-hidden dark:bg-app-dark-800"
    >
      <div
        class="w-full min-h-0 flex-1 overflow-y-auto overscroll-contain p-6"
        :style="kbInset > 0 ? { paddingBottom: `${kbInset}px` } : undefined"
      >
        <slot />
      </div>
    </main>
  </div>
</template>
