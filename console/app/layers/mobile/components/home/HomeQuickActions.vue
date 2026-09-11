<script setup lang="ts">
const actions = [
  {
    label: "New Note",
    icon: "heroicons:document-text-solid",
    href: "/notes/create-notes",

    iconColor: "text-violet-500",
  },
  {
    label: "New Task",
    icon: "heroicons:check-circle-solid",
    href: "/todo",

    iconColor: "text-emerald-500",
  },
  {
    label: "New Bookmark",
    icon: "heroicons:bookmark-solid",
    href: "/bookmarks",

    iconColor: "text-primary-500",
  },
];
</script>

<template>
  <div class="mt-5 flex items-center justify-around gap-3">
    <NuxtLink
      v-for="action in actions"
      :key="action.label"
      :to="action.href"
      class="flex flex-col items-center gap-1.5"
    >
      <div
        class="flex size-12 items-center justify-center rounded-2xl transition-transform active:scale-95"
        :class="action.bg"
      >
        <UIcon :name="action.icon" class="size-5" :class="action.iconColor" />
      </div>
      <span
        class="text-[11px] font-medium text-gray-500 dark:text-gray-400"
      >
        {{ action.label }}
      </span>
    </NuxtLink>
  </div>
</template>
