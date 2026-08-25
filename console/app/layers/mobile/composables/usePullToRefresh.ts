interface UsePullToRefreshOptions {
  onRefresh: () => Promise<void>;
  threshold?: number;
}

export function usePullToRefresh(options: UsePullToRefreshOptions) {
  const { onRefresh, threshold = 64 } = options;

  const container = ref<HTMLElement | null>(null);
  const isRefreshing = ref(false);
  const pullDistance = ref(0);

  let startY = 0;
  let pulling = false;

  function onTouchStart(e: TouchEvent) {
    if (isRefreshing.value) return;
    const el = container.value;
    if (!el || el.scrollTop > 0) return;

    startY = e.touches[0].clientY;
    pulling = true;
  }

  function onTouchMove(e: TouchEvent) {
    if (!pulling || isRefreshing.value) return;

    const currentY = e.touches[0].clientY;
    const delta = currentY - startY;

    if (delta > 0) {
      pullDistance.value = Math.min(delta * 0.5, threshold * 1.5);
    }
  }

  async function onTouchEnd() {
    if (!pulling) return;
    pulling = false;

    if (pullDistance.value >= threshold && !isRefreshing.value) {
      isRefreshing.value = true;
      pullDistance.value = threshold * 0.6;

      try {
        await onRefresh();
      } finally {
        isRefreshing.value = false;
        pullDistance.value = 0;
      }
    } else {
      pullDistance.value = 0;
    }
  }

  onMounted(() => {
    const el = container.value;
    if (!el) return;
    el.addEventListener("touchstart", onTouchStart, { passive: true });
    el.addEventListener("touchmove", onTouchMove, { passive: true });
    el.addEventListener("touchend", onTouchEnd, { passive: true });
  });

  onUnmounted(() => {
    const el = container.value;
    if (!el) return;
    el.removeEventListener("touchstart", onTouchStart);
    el.removeEventListener("touchmove", onTouchMove);
    el.removeEventListener("touchend", onTouchEnd);
  });

  return { container, isRefreshing, pullDistance };
}
