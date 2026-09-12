<script setup lang="ts">
import type { NotificationType } from "lunar";
import type { Notification } from "@shared/stores/notifications";
import { useTimeAgo } from "@vueuse/core";

const props = defineProps<{ notification: Notification }>();

const emit = defineEmits<{
  read: [identifier: string];
  dismiss: [identifier: string];
}>();

const timeAgo = useTimeAgo(props.notification.createdAt);

type NotificationCategory = "activity" | "system" | "alert";

const categoryConfig: Record<
  NotificationCategory,
  { icon: string; color: string; bg: string }
> = {
  activity: {
    icon: "heroicons:bolt",
    color: "text-surface-500 dark:text-surface-300",
    bg: "bg-surface-100 dark:bg-surface-900",
  },
  system: {
    icon: "heroicons:cog-6-tooth",
    color: "text-gray-500 dark:text-gray-400",
    bg: "bg-gray-100 dark:bg-gray-800",
  },
  alert: {
    icon: "heroicons:exclamation-triangle",
    color: "text-primary-500 dark:text-primary-400",
    bg: "bg-primary-50 dark:bg-primary-950",
  },
};

function getCategory(type: NotificationType): NotificationCategory {
  if (type === "backup_failed") return "alert";
  if (type.startsWith("workspace_invite") || type.startsWith("item"))
    return "activity";
  return "system";
}
</script>

<template>
  <div
    role="button"
    :tabindex="notification.isRead ? -1 : 0"
    class="group relative flex items-start gap-3 rounded-xl border bg-white p-4 transition-all active:scale-[0.98] dark:bg-gray-800"
    :class="
      notification.isRead
        ? 'border-gray-100 dark:border-gray-700'
        : 'cursor-pointer border-primary-100 shadow-sm hover:border-primary-200 dark:border-primary-900 dark:hover:border-primary-800'
    "
    @click="!notification.isRead && emit('read', notification.identifier)"
    @keydown.enter="
      !notification.isRead && emit('read', notification.identifier)
    "
  >
    <span
      v-if="!notification.isRead"
      class="absolute right-4 top-4 size-2 shrink-0 rounded-full bg-primary-500"
      aria-label="Unread"
    />

    <div
      class="flex size-8 shrink-0 items-center justify-center rounded-lg"
      :class="categoryConfig[getCategory(notification.notificationType)].bg"
    >
      <UIcon
        :name="categoryConfig[getCategory(notification.notificationType)].icon"
        class="size-4"
        :class="
          categoryConfig[getCategory(notification.notificationType)].color
        "
      />
    </div>

    <div class="min-w-0 flex-1 pr-6">
      <div class="flex items-baseline gap-2">
        <h3
          class="truncate text-sm"
          :class="
            notification.isRead
              ? 'font-normal text-gray-600 dark:text-gray-400'
              : 'font-medium text-gray-800 dark:text-gray-100'
          "
        >
          {{ notification.title }}
        </h3>
        <time class="shrink-0 text-[11px] text-gray-400 dark:text-gray-500">
          {{ timeAgo }}
        </time>
      </div>

      <p
        class="mt-0.5 line-clamp-2 text-xs leading-snug text-gray-400 dark:text-gray-500"
      >
        {{ notification.body }}
      </p>
    </div>

    <button
      class="absolute bottom-3.5 right-3.5 rounded-md p-1 text-gray-300 transition-colors hover:text-gray-500 dark:hover:text-gray-300"
      aria-label="Dismiss notification"
      @click.stop="emit('dismiss', notification.identifier)"
    >
      <UIcon name="heroicons:x-mark" class="size-3.5" />
    </button>
  </div>
</template>
