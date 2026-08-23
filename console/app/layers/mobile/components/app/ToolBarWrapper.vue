<script setup lang="ts"></script>

<template>
  <nav
    class="fixed bottom-0 inset-x-0 z-50 flex items-center justify-around border-t border-gray-200 dark:border-gray-800 bg-white dark:bg-app-dark-900 pt-1.5 max-h-20"
    style="padding-bottom: max(0.625rem, env(safe-area-inset-bottom))"
  >
    <slot />
  </nav>
</template>
