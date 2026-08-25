<template>
  <div class="flex gap-x-1.5">
    <UButton
      size="md"
      color="neutral"
      variant="ghost"
      icon="heroicons:magnifying-glass"
      aria-label="Notifications"
      @click="navigateTo('/notifications')"
    />

    <UColorModeButton />

    <UButton
      size="md"
      color="neutral"
      variant="ghost"
      icon="heroicons:bell"
      aria-label="Notifications"
      @click="navigateTo('/notifications')"
    />
  </div>
</template>
