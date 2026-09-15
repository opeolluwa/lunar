<script lang="ts" setup>
import { kFab } from "konsta/vue";
withDefaults(defineProps<{ icon?: string }>(), { icon: "heroicons:plus" });
</script>
<template>
  <kFab
    component="button"
    class="fixed bottom-24 right-7 z-10"
    :colors="{
      bgIos: 'bg-primary-500 dark:bg-primary-600',
      bgMaterial: 'bg-primary-500 dark:bg-primary-600',
      textIos: 'text-white',
      textMaterial: 'text-white',
    }"
  >
    <template #icon>
      <slot>
        <UIcon :name="icon" class="size-6" />
      </slot>
    </template>
  </kFab>
</template>
