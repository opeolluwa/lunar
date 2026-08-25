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

const walkthroughSeen = useLocalStorage("walkthroughSeen", false);
const route = useRoute();

if (!walkthroughSeen.value && route.path !== "/walkthrough") {
  navigateTo("/walkthrough");
}
</script>

<style></style>
