<script setup lang="ts">
import { items, secondaryRoutes } from "@shared/data/routes";

withDefaults(
  defineProps<{
    collapsed?: boolean;
  }>(),
  { collapsed: false },
);

const emit = defineEmits<{
  navigate: [];
}>();

const route = useRoute();
const colorMode = useColorMode();

const isDark = computed({
  get: () => colorMode.value === "dark",
  set: (value) => {
    colorMode.preference = value ? "dark" : "light";
  },
});

function toggleTheme() {
  isDark.value = !isDark.value;
}

const themeIcon = computed(() =>
  isDark.value ? "heroicons:sun" : "heroicons:moon",
);

const themeLabel = computed(() => (isDark.value ? "Light mode" : "Dark mode"));

function isActive(path: string): boolean {
  if (path === "/") {
    return route.path === "/";
  }

  return route.path === path || route.path.startsWith(`${path}/`);
}

function onNavigate() {
  emit("navigate");
}
</script>

<template>
  <div class="flex flex-col flex-1 min-h-0">
    <!-- Body -->
    <div class="flex flex-col min-h-0 overflow-y-auto px-3 pt-2 pb-1">
      <template
        v-for="(item, index) in items"
        :key="item.type === 'label' ? `label-${index}` : item.path"
      >
        <!-- LABEL -->
        <div
          v-if="item.type === 'label' && !collapsed"
          class="mt-6 px-2 mb-1.5 text-[11px] font-medium uppercase tracking-[0.14em] text-gray-400 dark:text-gray-500"
        >
          {{ item.name }}
        </div>

        <!-- LINK -->
        <UTooltip v-else :text="item.name" :disabled="!collapsed" side="right">
          <NuxtLink
            :to="item.path"
            class="flex items-center w-full h-11 mt-1 gap-2 px-2 rounded-xl transition-colors"
            :class="[
              collapsed ? 'justify-center px-0' : '',
              isActive(item.path)
                ? 'bg-primary-50 dark:bg-primary-950/40 text-primary-700 dark:text-primary-400 font-medium'
                : 'text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-white/[0.05] hover:text-gray-900 dark:hover:text-white',
            ]"
            @click="onNavigate"
          >
            <UIcon
              :name="isActive(item.path) ? item.activeIcon : item.icon"
              class="size-5 shrink-0"
              :class="
                isActive(item.path)
                  ? 'text-primary-500'
                  : 'text-gray-500 dark:text-gray-400'
              "
            />

            <span v-if="!collapsed" class="text-sm truncate">
              {{ item.name }}
            </span>
          </NuxtLink>
        </UTooltip>
      </template>
    </div>

    <!-- Footer -->
    <div class="shrink-0 px-3 pt-1 pb-4">
      <USeparator class="mb-3" />

      <!-- THEME -->
      <UTooltip :text="themeLabel" :disabled="!collapsed" side="right">
        <button
          type="button"
          class="flex items-center w-full h-11 gap-2 px-2 rounded-xl text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-white/[0.05] hover:text-gray-900 dark:hover:text-white transition-colors"
          :class="collapsed ? 'justify-center px-0' : ''"
          @click="toggleTheme"
        >
          <UIcon
            :name="themeIcon"
            class="size-5 shrink-0 text-gray-500 dark:text-gray-400"
          />

          <span v-if="!collapsed" class="text-sm font-medium">
            {{ themeLabel }}
          </span>
        </button>
      </UTooltip>

      <!-- SECONDARY -->
      <template v-for="item in secondaryRoutes" :key="item.path">
        <UTooltip :text="item.name" :disabled="!collapsed" side="right">
          <NuxtLink
            :to="item.path"
            class="flex items-center w-full h-11 mt-1 gap-2 px-2 rounded-xl transition-colors"
            :class="[
              collapsed ? 'justify-center px-0' : '',
              isActive(item.path)
                ? 'bg-primary-50 dark:bg-primary-950/40 text-primary-700 dark:text-primary-400 font-medium'
                : 'text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-white/[0.05] hover:text-gray-900 dark:hover:text-white',
            ]"
            @click="onNavigate"
          >
            <UIcon
              :name="isActive(item.path) ? item.activeIcon : item.icon"
              class="size-5 shrink-0"
              :class="
                isActive(item.path)
                  ? 'text-primary-500'
                  : 'text-gray-500 dark:text-gray-400'
              "
            />

            <span v-if="!collapsed" class="text-sm truncate">
              {{ item.name }}
            </span>
          </NuxtLink>
        </UTooltip>
      </template>
    </div>
  </div>
</template>
