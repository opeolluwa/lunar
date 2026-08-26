<script setup lang="ts">
defineProps<{
  userName: string;
  activeTodoCount: number;
  stats: {
    label: string;
    value: number;
    icon: string;
    color: string;
    href: string;
  }[];
}>();

const now = ref(new Date());
let timer: ReturnType<typeof setInterval>;

onMounted(() => {
  timer = setInterval(() => {
    now.value = new Date();
  }, 60_000);
});
onUnmounted(() => clearInterval(timer));

const greeting = computed(() => {
  const h = now.value.getHours();
  if (h < 12) return "Good morning";
  if (h < 17) return "Good afternoon";
  return "Good evening";
});

const today = computed(() =>
  now.value.toLocaleDateString("en-US", {
    weekday: "long",
    month: "long",
    day: "numeric",
  }),
);
</script>

<template>
  <div
    class="relative -mx-6 -mt-6 px-6 pt-7 pb-6 overflow-hidden bg-linear-to-br from-primary-500/10 via-violet-400/5 to-transparent dark:from-primary-500/12 dark:via-violet-500/6 dark:to-transparent border-b border-gray-100 dark:border-gray-800"
  >
    <div
      class="pointer-events-none absolute -top-10 right-0 size-52 rounded-full bg-primary-300/20 dark:bg-primary-500/10 blur-3xl"
    />
    <div
      class="pointer-events-none absolute bottom-0 left-1/2 size-36 rounded-full bg-violet-300/15 dark:bg-violet-500/8 blur-2xl translate-y-1/2"
    />

    <div class="relative flex items-end justify-between gap-4">
      <div>
        <p
          class="text-xs font-semibold tracking-widest uppercase text-gray-400 dark:text-gray-500 mb-1.5"
        >
          {{ today }}
        </p>
        <h1
          class="text-3xl font-bold tracking-tight text-gray-900 dark:text-white"
        >
          {{ greeting }}, {{ userName }}
        </h1>
        <p class="mt-1.5 text-sm text-gray-500 dark:text-gray-400">
          <template v-if="activeTodoCount > 0">
            <strong class="text-gray-700 dark:text-gray-300">{{
              activeTodoCount
            }}</strong>
            active {{ activeTodoCount === 1 ? "todo" : "todos" }} today.
          </template>
          <template v-else> You're all caught up. </template>
        </p>
      </div>
    </div>

    <div class="relative flex flex-wrap items-center gap-2 mt-5">
      <NuxtLink
        v-for="s in stats"
        :key="s.label"
        :to="s.href"
        class="flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-medium bg-white/70 dark:bg-gray-900/60 border border-gray-200/80 dark:border-gray-700/60 backdrop-blur-sm hover:border-primary-300 dark:hover:border-primary-700 transition-colors"
      >
        <UIcon :name="s.icon" class="size-3.5 shrink-0" :class="s.color" />
        <span class="text-gray-800 dark:text-gray-200 tabular-nums">{{
          s.value
        }}</span>
        <span class="text-gray-400 dark:text-gray-500">{{ s.label }}</span>
      </NuxtLink>
    </div>
  </div>
</template>
