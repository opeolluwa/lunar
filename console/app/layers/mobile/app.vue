<template>
  <UApp id="lunar_mobile">
    <kApp :theme="currentPlatform === 'ios' ? 'ios' : 'material'">
      <NuxtLayout class="safe-areas">
        <NuxtPage />
      </NuxtLayout>
    </kApp>
  </UApp>
</template>

<script lang="ts" setup>
import { kApp } from "konsta/vue";
const currentPlatform = usePlatform();
</script>

<style>
/*.fade-enter-active,
.fade-leave-active {
  transition: opacity 200ms ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}*/
</style>
