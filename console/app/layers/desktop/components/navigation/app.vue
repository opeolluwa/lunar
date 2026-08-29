<script setup lang="ts">
import { items, secondaryRoutes } from "@shared/data/routes";
import { useSidebarStore } from "@shared/stores/sidebar";

const route = useRoute();
const colorMode = useColorMode();
const sidebarStore = useSidebarStore();

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

const themeLabel = computed(() =>
  isDark.value ? "Light mode" : "Dark mode",
);

function isActive(path: string) {
  if (path === "/") {
    return route.path === "/";
  }

  return route.path.startsWith(path);
}
</script>

<template>
  <UDashboardSidebar
    id="app"
    v-model:collapsed="sidebarStore.collapsed"
    class="hidden md:flex"
    :collapsible="true"
    :collapsed-size="4"
    :default-size="18"
    :min-size="4"
    :max-size="42"
    resizable
    :ui="{
      root: [
        'bg-white dark:bg-gray-950',
        'border-e border-gray-200 dark:border-gray-800',
        'overflow-hidden',
      ],
      header: 'shrink-0 h-auto p-0',
      body: 'flex flex-col flex-1 min-h-0 p-0',
      footer: 'shrink-0 p-0',
      handle: 'cursor-ew-resize',
    }"
  >
    <!-- HEADER -->
    <template #header>
      <div
        class="flex items-center h-[76px]"
        :class="
          sidebarStore.collapsed
            ? 'justify-center'
            : 'justify-start pl-5'
        "
      >
        <UDashboardSidebarCollapse
          :icon="
            sidebarStore.collapsed
              ? 'i-lucide-panel-left-open'
              : 'i-lucide-panel-left-close'
          "
          class="text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-white"
        />
      </div>
    </template>

    <!-- RESIZE HANDLE -->
    <template #resize-handle="{ onMouseDown, onTouchStart, onDoubleClick }">
      <UDashboardResizeHandle
        class="after:absolute after:inset-y-0 after:right-0 after:w-px hover:after:bg-(--ui-border-primary) after:transition"
        @mousedown="onMouseDown"
        @touchstart="onTouchStart"
        @dblclick="onDoubleClick"
      />
    </template>

    <!-- BODY -->
    <template #default>
      <div class="flex flex-col flex-1 min-h-0 overflow-y-auto px-3 pt-2">
        <template
          v-for="(item, index) in items"
          :key="item.type === 'label' ? `label-${index}` : item.path"
        >
          <!-- LABEL -->
          <div
            v-if="item.type === 'label' && !sidebarStore.collapsed"
            class="mt-6 px-2 mb-1.5 text-[11px] font-medium uppercase tracking-[0.14em] text-gray-400 dark:text-gray-500"
          >
            {{ item.name }}
          </div>

          <!-- LINK -->
          <UTooltip
            v-else
            :text="item.name"
            :disabled="!sidebarStore.collapsed"
            side="right"
          >
            <NuxtLink
              :to="item.path"
              class="flex items-center w-full h-11 mt-1 gap-2 px-2 rounded-xl transition-colors"
              :class="[
                sidebarStore.collapsed ? 'justify-center px-0' : '',
                isActive(item.path)
                  ? 'bg-primary-50 dark:bg-primary-950/40 text-primary-700 dark:text-primary-400 font-medium'
                  : 'text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-white/[0.05] hover:text-gray-900 dark:hover:text-white',
              ]"
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

              <span
                v-if="!sidebarStore.collapsed"
                class="text-sm truncate"
              >
                {{ item.name }}
              </span>
            </NuxtLink>
          </UTooltip>
        </template>
      </div>
    </template>

    <!-- FOOTER -->
    <template #footer>
      <div class="px-3 pb-4">
        <USeparator class="mb-3" />

        <!-- SECONDARY -->
        <template v-for="item in secondaryRoutes" :key="item.path">
          <UTooltip
            :text="item.name"
            :disabled="!sidebarStore.collapsed"
            side="right"
          >
            <NuxtLink
              :to="item.path"
              class="flex items-center w-full h-11 mt-1 gap-2 px-2 rounded-xl transition-colors"
              :class="[
                sidebarStore.collapsed ? 'justify-center px-0' : '',
                isActive(item.path)
                  ? 'bg-primary-50 dark:bg-primary-950/40 text-primary-700 dark:text-primary-400 font-medium'
                  : 'text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-white/[0.05] hover:text-gray-900 dark:hover:text-white',
              ]"
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

              <span
                v-if="!sidebarStore.collapsed"
                class="text-sm truncate"
              >
                {{ item.name }}
              </span>
            </NuxtLink>
          </UTooltip>
        </template>

        <!-- THEME -->
        <UTooltip
          :text="themeLabel"
          :disabled="!sidebarStore.collapsed"
          side="right"
        >
          <button
            type="button"
            class="flex items-center w-full h-11 gap-2 px-2 rounded-xl text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-white/[0.05] hover:text-gray-900 dark:hover:text-white transition-colors"
            :class="sidebarStore.collapsed ? 'justify-center px-0' : ''"
            @click="toggleTheme"
          >
            <UIcon
              :name="themeIcon"
              class="size-5 shrink-0 text-gray-500 dark:text-gray-400"
            />

            <span
              v-if="!sidebarStore.collapsed"
              class="text-sm font-medium"
            >
              {{ themeLabel }}
            </span>
          </button>
        </UTooltip>
      </div>
    </template>
  </UDashboardSidebar>
</template>