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

    <UUser
      size="sm"
      class="cursor-pointer"
      :avatar="{ src: 'https://i.pravatar.cc/150?u=john-doe' }"
    />
  </div>
</template>
