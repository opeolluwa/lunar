<script setup lang="ts">
defineProps({
  hideHeaderAndNav: Boolean,
});

const kbInset = useKeyboardInset();
</script>
<template>
  <div
    id="viewport_mobile"
    class="px-6 flex-1 overflow-y-scroll min-h-0 pt-8 pb-32"
    :style="kbInset > 0 ? { paddingBottom: `${kbInset}px` } : undefined"
  >
    <slot />
  </div>
</template>
