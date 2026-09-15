<script setup lang="ts">
import { useAuthStore } from "@shared/stores/auth";

definePageMeta({ layout: "auth" });

const authStore = useAuthStore();
const {
  flow,
  otp,
  errors,
  loading,
  submitError,
  remaining,
  handleSubmit,
  handleResend,
} = useConfirmOtp();

const title = computed(() =>
  flow.value === "reset" ? "Verify reset code" : "Confirm your email",
);

const description = computed(() =>
  flow.value === "reset"
    ? "Enter the 6-digit code we sent to your email to reset your password."
    : "Enter the 6-digit verification code we sent to your email to activate your account.",
);
</script>

<template>
  <div class="flex flex-col gap-5">
    <AppPageHeader :title="title" :description="description" />

    <form class="flex flex-col gap-4" @submit.prevent="handleSubmit">
      <div>
        <AppNumberInput
          v-model="otp"
          label="Verification code"
          name="otp"
          placeholder="••••••"
          :disabled="loading"
          inputmode="numeric"
          autocomplete="one-time-code"
          :maxlength="6"
        />
        <div class="flex justify-end mt-1">
          <button
            type="button"
            :disabled="remaining > 0 || !authStore.hasPendingToken"
            class="text-xs text-primary-500 hover:text-primary-600 font-medium disabled:opacity-40 disabled:cursor-not-allowed"
            @click="handleResend"
          >
            {{
              remaining > 0
                ? `Get new code after ${remaining}s`
                : "Get new code"
            }}
          </button>
        </div>
      </div>
      <p v-if="errors.otp" class="text-xs text-red-500 -mt-3">
        {{ errors.otp }}
      </p>

      <p v-if="submitError" class="text-sm text-red-500">{{ submitError }}</p>

      <AppButton
        type="submit"
        color="primary"
        class="w-full py-3 bg-primary-500 hover:bg-primary-600 rounded-lg text-white font-medium disabled:opacity-50 text-center"
        :loading="loading"
        :disabled="loading"
      >
        Verify code
      </AppButton>
    </form>

    <p class="text-sm text-center text-gray-500 dark:text-gray-400">
      {{ flow === "reset" ? "Remembered your password?" : "Already verified?" }}
      <NuxtLink
        :to="flow === 'reset' ? '/auth/login' : '/auth/login'"
        class="text-primary-500 hover:text-primary-600 font-medium"
      >
        Sign in
      </NuxtLink>
    </p>
  </div>
</template>
