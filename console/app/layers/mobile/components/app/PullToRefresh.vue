<script setup lang="ts">
import { kPreloader } from "konsta/vue";

const props = withDefaults(defineProps<{ threshold?: number }>(), {
  threshold: 64,
});

const emit = defineEmits<{ refresh: [] }>();

const { container, isRefreshing, pullDistance } = usePullToRefresh({
  onRefresh: () => emit("refresh"),
  threshold: props.threshold,
});
</script>

<template>
  <div ref="container">
    <Transition name="slide-down">
      <div
        v-if="isRefreshing || pullDistance > 0"
        class="flex justify-center py-2"
        :style="{ height: `${pullDistance}px` }"
      >
        <kPreloader
          :size="pullDistance >= 64 && isRefreshing ? 'w-6 h-6' : 'w-5 h-5'"
          class="text-primary-500"
        />
      </div>
    </Transition>
    <slot />
  </div>
</template>

<style scoped>
.slide-down-enter-active,
.slide-down-leave-active {
  transition: all 0.2s ease-out;
}
.slide-down-enter-from,
.slide-down-leave-to {
  opacity: 0;
  transform: translateY(-8px);
}
</style>
