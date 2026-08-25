<template>
  <UApp id="lunar_mobile">
     <kApp :theme="framework7Theme"> 
      <NuxtLayout>
        <NuxtPage :key="$route.fullPath" />
      </NuxtLayout>
    </kApp>
  </UApp>
</template>

<script lang="ts" setup>
import { kApp } from "konsta/vue";
const { framework7Theme } = usePlatform();
</script>

<style></style>
