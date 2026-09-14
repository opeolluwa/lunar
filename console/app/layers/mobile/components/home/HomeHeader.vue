<script setup lang="ts">
import { useTodoStore } from "@shared/stores/todo";
import { useUserPreferenceStore } from "@shared/stores/workspace-profile";

const todoStore = useTodoStore();
const userPreferenceStore = useUserPreferenceStore();

const now = ref(new Date());
let clockTimer: ReturnType<typeof setInterval>;

onMounted(() => {
  clockTimer = setInterval(() => {
    now.value = new Date();
  }, 60_000);
});
onUnmounted(() => clearInterval(clockTimer));

const greeting = computed(() => {
  const h = now.value.getHours();
  if (h < 12) return "Good morning";
  if (h < 17) return "Good afternoon";
  return "Good evening";
});

const today = computed(() =>
  now.value.toLocaleDateString("en-US", {
    weekday: "short",
    month: "short",
    day: "numeric",
  }),
);

const firstName = computed(
  () => userPreferenceStore.preference?.firstName || "there",
);
</script>

<template>
  <div class="pt-1">
    <h1
      class="text-[26px] font-bold leading-tight tracking-tight text-gray-900 dark:text-white"
    >
      {{ greeting }},<br />
      {{ firstName }} 👋
    </h1>
    <p class="mt-1.5 text-[13px] text-gray-500 dark:text-gray-400">
      <template v-if="todoStore.activeTodos.length > 0">
        <strong class="text-gray-700 dark:text-gray-300">{{
          todoStore.activeTodos.length
        }}</strong>
        active
        {{ todoStore.activeTodos.length === 1 ? "todo" : "todos" }} today.
      </template>
      <template v-else> You're all caught up. </template>
    </p>
  </div>
</template>
