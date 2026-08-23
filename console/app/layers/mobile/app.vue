<template>
  <UApp id="lunar_mobile">
    <kProvider :theme="framework7Theme">
      <f7App :theme="framework7Theme">
        <NuxtLayout class="safe-areas">
          <NuxtPage />
        </NuxtLayout>
      </f7App>
    </kProvider>
  </UApp>
</template>

<script lang="ts" setup>
import { kProvider } from "konsta/vue";
import { f7App } from "framework7-vue";
const { framework7Theme } = usePlatform();
</script>

<style></style>
