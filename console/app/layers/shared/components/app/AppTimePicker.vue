<script setup lang="ts">
export type TimeValue = { hour: number; minute: number };

const props = defineProps<{
  modelValue: TimeValue | null | undefined;
  disabled?: boolean;
}>();

const emit = defineEmits<{
  "update:modelValue": [value: TimeValue | null];
}>();

const hours = Array.from({ length: 12 }, (_, i) => i + 1);
const minutes = Array.from({ length: 12 }, (_, i) => i * 5);

function pad(value: number): string {
  return String(value).padStart(2, "0");
}

function displayValue(): string {
  if (!props.modelValue) return "";
  return new Date(0, 0, 0, props.modelValue.hour, props.modelValue.minute)
    .toLocaleTimeString("en-US", { hour: "numeric", minute: "2-digit" });
}

function selection(): { hour: number; minute: number; period: "AM" | "PM" } {
  const v = props.modelValue;
  if (!v) return { hour: 1, minute: 0, period: "AM" };
  const hour12 = v.hour % 12 || 12;
  return {
    hour: hour12,
    minute: v.minute,
    period: v.hour >= 12 ? "PM" : "AM",
  };
}

function selectHour(hour: number) {
  const s = selection();
  setTime(hour, s.minute, s.period);
}

function selectMinute(minute: number) {
  const s = selection();
  setTime(s.hour, minute, s.period);
}

function selectPeriod(period: "AM" | "PM") {
  const s = selection();
  setTime(s.hour, s.minute, period);
}

function setTime(hour12: number, minute: number, period: "AM" | "PM") {
  const hour24 = period === "PM" ? (hour12 % 12) + 12 : hour12 % 12;
  emit("update:modelValue", { hour: hour24, minute });
}
</script>

<template>
  <UPopover>
    <button
      type="button"
      :disabled="disabled"
      class="w-full flex items-center gap-2 bg-white dark:bg-gray-800 rounded-lg px-4 py-2.5 text-sm border border-gray-200 dark:border-gray-700 hover:border-gray-300 dark:hover:border-gray-600 transition-colors text-left disabled:opacity-50"
      :class="modelValue ? 'text-gray-700 dark:text-gray-200' : 'text-gray-400 dark:text-gray-500'"
    >
      <UIcon name="heroicons:clock" class="size-4 shrink-0 text-gray-400" />
      {{ modelValue ? displayValue() : "Pick a time" }}
    </button>

    <template #content="{ close }">
      <div class="w-64 p-3 flex flex-col gap-3">
        <div class="grid grid-cols-2 gap-3">
          <div class="flex flex-col gap-1.5">
            <span class="text-[11px] font-medium text-gray-400 uppercase tracking-wide">
              Hour
            </span>
            <div class="grid grid-cols-4 gap-1">
              <button
                v-for="h in hours"
                :key="h"
                type="button"
                class="py-1.5 rounded-md text-xs font-medium transition-colors"
                :class="
                  selection().hour === h
                    ? 'bg-primary-500 text-white'
                    : 'bg-gray-100 dark:bg-gray-800 text-gray-600 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-700'
                "
                @click="selectHour(h)"
              >
                {{ h }}
              </button>
            </div>
          </div>

          <div class="flex flex-col gap-1.5">
            <span class="text-[11px] font-medium text-gray-400 uppercase tracking-wide">
              Minute
            </span>
            <div class="grid grid-cols-4 gap-1">
              <button
                v-for="m in minutes"
                :key="m"
                type="button"
                class="py-1.5 rounded-md text-xs font-medium transition-colors"
                :class="
                  selection().minute === m
                    ? 'bg-primary-500 text-white'
                    : 'bg-gray-100 dark:bg-gray-800 text-gray-600 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-700'
                "
                @click="selectMinute(m)"
              >
                {{ pad(m) }}
              </button>
            </div>
          </div>
        </div>

        <div class="flex flex-col gap-1.5">
          <span class="text-[11px] font-medium text-gray-400 uppercase tracking-wide">
            Period
          </span>
          <div class="grid grid-cols-2 gap-1">
            <button
              v-for="p in (['AM', 'PM'] as const)"
              :key="p"
              type="button"
              class="py-1.5 rounded-md text-xs font-medium transition-colors"
              :class="
                selection().period === p
                  ? 'bg-primary-500 text-white'
                  : 'bg-gray-100 dark:bg-gray-800 text-gray-600 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-700'
              "
              @click="selectPeriod(p)"
            >
              {{ p }}
            </button>
          </div>
        </div>

        <button
          type="button"
          class="w-full py-2 rounded-lg text-xs font-medium bg-primary-500 text-white hover:bg-primary-600 transition-colors"
          @click="close"
        >
          Done
        </button>
      </div>
    </template>
  </UPopover>
</template>