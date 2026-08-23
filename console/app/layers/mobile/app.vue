<template>
  <kApp :theme="framework7Theme">
    <UApp id="lunar_mobile">
      <NuxtLayout class="safe-areas">
        <NuxtPage />
      </NuxtLayout>
    </UApp>
  </kApp>
</template>

<script lang="ts" setup>
import { kApp } from "konsta/vue";

const { framework7Theme } = usePlatform();
</script>

<style></style>
