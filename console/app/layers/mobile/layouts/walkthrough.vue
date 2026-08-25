<script lang="ts" setup>
import { kPage } from "konsta/vue";
</script>

<template>
  <kPage
    class="h-dvh overflow-hidden flex flex-col bg-white dark:bg-app-dark-800"
  >
    <slot />
  </kPage>
</template>
