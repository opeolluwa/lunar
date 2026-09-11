<script setup lang="ts">
import { useNotificationStore } from "@shared/stores/notifications";
import EmptyState from "@shared/components/app/EmptyState.vue";

definePageMeta({ name: "Notifications" });

const notificationStore = useNotificationStore();

onMounted(() => notificationStore.fetchNotifications());

function handleRead(identifier: string) {
  notificationStore.markAsRead(identifier);
}

function handleDismiss(identifier: string) {
  notificationStore.deleteNotification(identifier);
}
</script>

<template>
  <div class="flex flex-col">
    <div v-if="notificationStore.loading" class="mt-3 flex flex-col gap-2">
      <USkeleton v-for="i in 4" :key="i" class="h-16 rounded-xl" />
    </div>

    <EmptyState
      v-else-if="notificationStore.notifications.length === 0"
      title="You're all caught up"
      description="New notifications will appear here."
      icon="heroicons:bell-slash"
    />

    <div v-else class="mt-3 flex flex-col gap-2">
      <NotificationsCard
        v-for="notification in notificationStore.notifications"
        :key="notification.identifier"
        :notification="notification"
        @read="handleRead"
        @dismiss="handleDismiss"
      />
    </div>
  </div>
</template>
