<script lang="ts" setup>
import { primaryRoutes, secondaryRoutes } from "@shared/data/routes";

import { kNavbar, kNavbarBackLink, kPage } from "konsta/vue";

const route = useRoute();
const router = useRouter();

const { toggleMobileNav } = useMobileNav();

const topLevelPaths = [...primaryRoutes, ...secondaryRoutes].map(
  (item) => item.path,
);

const isTopLevel = computed(() => topLevelPaths.includes(route.path));

const hideHeaderAndNav = computed(() => {
  return (
    route.path.includes("/create-notes") || route.path.includes("/edit-notes")
  );
});

const pageTitle = computed(() => route.meta.name);

useHead({ title: () => pageTitle.value as string });
</script>

<template>
  <div>
    <kNavbar bg-class="bg-white dark:bg-app-dark-800" class="px-2">
      <template #title>
        <span class="font-medium text-lg">{{ pageTitle }}</span>
      </template>

      <template #left>
        <UButton
          v-if="isTopLevel"
          size="md"
          color="neutral"
          variant="ghost"
          icon="heroicons:bars-3"
          aria-label="Open menu"
          @click="toggleMobileNav"
        />

        <kNavbarBackLink
          v-else
          text="Back"
          component="div"
          @click="router.back()"
        />
      </template>

      <template #right>
        <UButton
          size="md"
          color="neutral"
          variant="ghost"
          icon="heroicons:bell"
          aria-label="Notifications"
          @click="navigateTo('/notifications')"
        />
      </template>
    </kNavbar>

    <main
      id="default_layout_mobile"
      class="flex h-dvh flex-col overflow-hidden bg-gray-50 dark:bg-app-dark-800"
    >
      <AppViewport :hide-header-and-nav="hideHeaderAndNav">
        <slot />
        Lorem ipsum dolor sit amet consectetur adipisicing elit. Quod rem odit natus maiores earum sapiente distinctio tempore sunt beatae, qui quisquam at nesciunt alias blanditiis fugiat accusamus similique placeat odio?
        Lorem ipsum dolor sit, amet consectetur adipisicing elit. Quaerat vero possimus sit, quis repudiandae excepturi dolore accusamus magni ullam temporibus! Delectus unde reprehenderit sint esse, vel doloribus ipsam facilis ipsum!
        Lorem ipsum dolor sit amet consectetur adipisicing elit. Quod rem odit natus maiores earum sapiente distinctio tempore sunt beatae, qui quisquam at nesciunt alias blanditiis fugiat accusamus similique placeat odio?
        Lorem ipsum dolor sit, amet consectetur adipisicing elit. Quaerat vero possimus sit, quis repudiandae excepturi dolore accusamus magni ullam temporibus! Delectus unde reprehenderit sint esse, vel doloribus ipsam facilis ipsum! Lorem ipsum dolor sit amet consectetur adipisicing elit. Quod rem odit natus maiores earum sapiente distinctio tempore sunt beatae, qui quisquam at nesciunt alias blanditiis fugiat accusamus similique placeat odio?
        Lorem ipsum dolor sit, amet consectetur adipisicing elit. Quaerat vero possimus sit, quis repudiandae excepturi dolore accusamus magni ullam temporibus! Delectus unde reprehenderit sint esse, vel doloribus ipsam facilis ipsum! Lorem ipsum dolor sit amet consectetur adipisicing elit. Quod rem odit natus maiores earum sapiente distinctio tempore sunt beatae, qui quisquam at nesciunt alias blanditiis fugiat accusamus similique placeat odio?
        Lorem ipsum dolor sit, amet consectetur adipisicing elit. Quaerat vero possimus sit, quis repudiandae excepturi dolore accusamus magni ullam temporibus! Delectus unde reprehenderit sint esse, vel doloribus ipsam facilis ipsum! Lorem ipsum dolor sit amet consectetur adipisicing elit. Quod rem odit natus maiores earum sapiente distinctio tempore sunt beatae, qui quisquam at nesciunt alias blanditiis fugiat accusamus similique placeat odio?
        Lorem ipsum dolor sit, amet consectetur adipisicing elit. Quaerat vero possimus sit, quis repudiandae excepturi dolore accusamus magni ullam temporibus! Delectus unde reprehenderit sint esse, vel doloribus ipsam facilis ipsum!
      </AppViewport>

      <AppBottonNav v-if="!hideHeaderAndNav" />

      <AppSideNav />
    </main>
  </div>
</template>
