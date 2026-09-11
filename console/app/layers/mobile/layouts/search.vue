<script lang="ts" setup>
import { kNavbar, kPage } from "konsta/vue";
const router = useRouter();
</script>

<template>
  <kPage class="h-dvh overflow-hidden flex flex-col">
    <kNavbar
      title="Search"
      :center-title="false"
      title-class="truncate text-md pl-4 font-medium"
      bg-class="bg-white dark:bg-app-dark-800"
      class="shrink-0 px-2"
    >
      <template #left>
        <button class="inline-flex items-center" @click="router.back()">
          <UIcon name="lucide:arrow-left" class="size-5" />
        </button>
      </template>
    </kNavbar>

    <main class="flex min-h-0 flex-1 flex-col overflow-hidden">
      <div class="min-h-0 flex-1 overflow-y-auto">
        <slot />
      </div>
    </main>
  </kPage>
</template>
