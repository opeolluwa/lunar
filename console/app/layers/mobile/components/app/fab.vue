<script lang="ts" setup>
import { kFab } from "konsta/vue";
</script>
<template>
  <kFab
    component="button"
    aria-label="Add note"
    class="absolute bottom-24 right-7 md:hidden"
    :colors="{
      bgIos: 'bg-primary-500 dark:bg-primary-600',
      bgMaterial: 'bg-primary-500 dark:bg-primary-600',
      textIos: 'text-white',
      textMaterial: 'text-white',
    }"
  >
    <template #icon>
      <slot>
        <UIcon name="heroicons:plus" class="size-6" />
      </slot>
    </template>
  </kFab>
</template>
