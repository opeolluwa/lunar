<script setup lang="ts">
import { useNotificationStore } from '@shared/stores/notifications';

definePageMeta({ name: "Notifications" });

const notificationStore = useNotificationStore();
const notifications = computed(() => notificationStore.notifications);

onMounted(() => notificationStore.fetchNotifications());
</script>

<template>
  <div class="flex flex-col">
    <NuxtLink href="/walkthrough">walk through</NuxtLink>
    <NuxtLink href="/auth/login">login</NuxtLink>

    <div class="space-y-3">
      <NotificationsCard
        v-for="notification in notifications"
        :key="notification.identifier"
        :notification="notification"
        @read="(identifier) => notificationStore.markAsRead(identifier)"
        @dismiss="(identifier) => notificationStore.deleteNotification(identifier)"
      />
    </div>
  </div>
</template>