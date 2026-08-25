<script setup lang="ts">
definePageMeta({ layout: "walkthrough", name: "Walkthrough" });

const walkthroughSeen = useLocalStorage("walkthroughSeen", false);

const currentSlide = ref(0);

const slides = [
  {
    icon: "heroicons:home",
    title: "Welcome to Lunar",
    description: "Your personal space for notes, tasks, and bookmarks.",
  },
  {
    icon: "heroicons:briefcase",
    title: "Organize with Workspaces",
    description:
      "Group related items into workspaces for different projects or contexts.",
  },
  {
    icon: "heroicons:rocket-launch",
    title: "Get Started",
    description: "Create your first workspace and start capturing your ideas.",
  },
];

const isLastSlide = computed(() => currentSlide.value === slides.length - 1);

function complete() {
  walkthroughSeen.value = true;
  navigateTo("/");
}

function next() {
  if (isLastSlide.value) {
    complete();
  } else {
    currentSlide.value++;
  }
}
</script>

<template>
  <div
    class="h-dvh flex flex-col justify-between px-6 pt-12 pb-[env(safe-area-inset-bottom)]"
  >
    <UCarousel
      v-slot="{ item }"
      v-model="currentSlide"
      :items="slides"
      :ui="{ item: 'basis-full' }"
      dots
      class="w-full flex-1 flex items-center"
    >
      <div class="flex flex-col items-center text-center gap-6 px-4">
        <div
          class="size-20 rounded-full bg-primary-50 dark:bg-primary-950 flex items-center justify-center"
        >
          <UIcon :name="item.icon" class="size-10 text-primary-500" />
        </div>
        <div class="flex flex-col gap-2">
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white">
            {{ item.title }}
          </h2>
          <p class="text-sm text-gray-500 dark:text-gray-400">
            {{ item.description }}
          </p>
        </div>
      </div>
    </UCarousel>

    <div class="flex flex-col gap-3 w-full">
      <AppButton @click="next">
        {{ isLastSlide ? "Get started" : "Next" }}
      </AppButton>
      <button
        v-if="!isLastSlide"
        type="button"
        class="text-sm text-gray-400 dark:text-gray-500 text-center"
        @click="complete"
      >
        Skip
      </button>
    </div>
  </div>
</template>
