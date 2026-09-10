<script setup lang="ts">
type ButtonType = "button" | "submit" | "reset" | "link";

type ButtonColor =
  | "primary"
  | "error"
  | "secondary"
  | "success"
  | "info"
  | "warning"
  | "neutral";

type ButtonSize = "xs" | "sm" | "md" | "lg" | "xl";

type ButtonVariant = "link" | "solid" | "outline" | "soft" | "subtle" | "ghost";

withDefaults(
  defineProps<{
    type?: ButtonType;
    color?: ButtonColor;
    loading?: boolean;
    disabled?: boolean;
    error?: boolean;
    size?: ButtonSize;
    variant?: ButtonVariant;
    full?: boolean;
  }>(),
  {
    type: "button",
    color: "primary",
    loading: false,
    disabled: false,
    error: false,
    size: "md",
    variant: "solid",
    full: true,
  },
);
</script>

<template>
  <UButton
    :type="type"
    :color="color"
    :loading="loading"
    :variant="variant"
    :size="size"
    :disabled="disabled || loading"
    :class="full ? 'w-full' : 'w-auto'"
    class="cursor-pointer bg-primary-500 py-3 text-center font-medium text-white hover:bg-primary-600 disabled:opacity-50"
  >
    <slot />
  </UButton>
</template>
