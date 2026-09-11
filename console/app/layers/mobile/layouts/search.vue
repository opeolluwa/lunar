<script lang="ts" setup>
import { kNavbar, kPage } from "konsta/vue";
const router = useRouter();
const { searchQuery } = useAppSearch();

function handleTrailing() {
  if (searchQuery.value) {
    searchQuery.value = "";
  }
}
</script>

<template>
  <kPage class="h-dvh overflow-hidden flex flex-col">
    <kNavbar
      :title="undefined"
      :center-title="false"
      title-class="flex-1 min-w-0"
      bg-class="bg-white dark:bg-app-dark-800"
      class="shrink-0 px-2"
    >
      <template #left>
        <button class="inline-flex items-center" @click="router.back()">
          <UIcon name="lucide:arrow-left" class="size-5" />
        </button>
      </template>

      <template #title>
        <UInput
          v-model="searchQuery"
          size="md"
          placeholder="Search..."
          variant="none"
          autofocus
          autocapitalize="off"
          autocorrect="off"
          spellcheck="false"
          :ui="{ trailing: 'pe-1', root: 'ml-4 border-none' }"
          @keydown.escape="handleTrailing"
        >
          <template #trailing>
            <UButton
              color="neutral"
              variant="link"
              size="sm"
              icon="i-lucide-circle-x"
              :aria-label="searchQuery ? 'Clear input' : 'Close search'"
              @click="handleTrailing"
            />
          </template>
        </UInput>
      </template>
    </kNavbar>

    <main class="flex min-h-0 flex-1 flex-col overflow-hidden">
      <div class="min-h-0 flex-1 overflow-y-auto">
        <slot />
      </div>
    </main>
  </kPage>
</template>
