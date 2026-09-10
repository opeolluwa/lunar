<script setup lang="ts">
definePageMeta({ layout: "walkthrough", name: "Walkthrough" });

const walkthroughSeen = useLocalStorage("walkthroughSeen", false);

const currentSlide = ref(0);

const slides = [
  {
    image: "everything-one-place",
    title: "Everything in one place",
    description:
      "Keep notes, tasks, and bookmarks together in your personal workspace.",
  },
  {
    image: "find-anything",
    title: "Find anything instantly",
    description:
      "Search across everything at once — no more digging through folders.",
  },
  {
    image: "ideas-captured",
    title: "Ideas, captured",
    description:
      "Turn thoughts into notes and reminders, and access it all anywhere.",
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

function skip() {
  complete();
}

let touchStartX = 0;

function onTouchStart(event: TouchEvent) {
  touchStartX = event.touches[0]?.clientX ?? 0;
}

function onTouchEnd(event: TouchEvent) {
  const touchEndX = event.changedTouches[0]?.clientX ?? 0;
  const distance = touchEndX - touchStartX;

  if (Math.abs(distance) < 50) {
    return;
  }

  next();
}
</script>

<template>
  <main
    class="h-dvh w-full flex flex-col bg-white dark:bg-gray-950 select-none"
  >
    <!-- Top bar -->
    <header
      class="shrink-0 flex items-center justify-between px-6 pt-safe-top pt-6"
    >
      <div class="flex ml-auto">
        <button
          v-if="!isLastSlide"
          type="button"
          class="text-sm font-medium text-gray-400 dark:text-gray-500"
          @click.stop="skip"
        >
          Skip
        </button>
      </div>
    </header>

    <!-- Scrollable slides -->
    <div
      class="flex-1 min-h-0 overflow-y-auto touch-pan-y scrollbar-config"
      @touchstart="onTouchStart"
      @touchend="onTouchEnd"
    >
      <div class="min-h-full flex items-center justify-center">
        <UCarousel
          v-slot="{ item, index }"
          v-model="currentSlide"
          :items="slides"
          :ui="{ item: 'basis-full' }"
          class="w-full"
        >
          <section class="flex flex-col items-center justify-center px-8 py-10">
            <div class="relative mb-10">
              <div
                class="absolute inset-0 -m-10 rounded-full bg-primary-500/10 blur-3xl"
              />

              <div
                class="relative w-52 md:w-60 aspect-square flex items-center justify-center"
              >
                <img
                  :src="`/${item.image}.svg`"
                  :alt="item.title"
                  class="w-full h-full object-contain"
                  draggable="false"
                />
              </div>
            </div>

            <div class="max-w-sm text-center">
   
              <h1
                class="text-3xl font-bold tracking-tight text-gray-950 dark:text-white"
              >
                {{ item.title }}
              </h1>

              <p
                class="mt-4 text-base leading-7 text-gray-500 dark:text-gray-400"
              >
                {{ item.description }}
              </p>
            </div>
          </section>
        </UCarousel>
      </div>
    </div>

    <!-- Bottom controls -->
    <footer class="shrink-0 px-6 pb-safe-bottom pb-8">
      <!-- Progress -->
      <div class="flex items-center justify-center gap-2 mb-7">
        <button
          v-for="(_, index) in slides"
          :key="index"
          type="button"
          class="h-1.5 rounded-full transition-all duration-300"
          :class="
            currentSlide === index
              ? 'w-7 bg-primary-500'
              : 'w-1.5 bg-gray-200 dark:bg-gray-700'
          "
          :aria-label="`Go to slide ${index + 1}`"
          @click.stop="currentSlide = index"
        />
      </div>

      <!-- Action -->
      <button
        type="button"
        class="w-full h-14 rounded-2xl bg-primary-600 hover:bg-primary-700 active:bg-primary-800 text-white font-semibold transition-colors flex items-center justify-center gap-2"
        @click.stop="next"
      >
        {{ isLastSlide ? "Get started" : "Continue" }}
        <UIcon
          v-if="!isLastSlide"
          name="heroicons:arrow-right"
          class="size-4"
        />
      </button>


    </footer>
  </main>
</template>
