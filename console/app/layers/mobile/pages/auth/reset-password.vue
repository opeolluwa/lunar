<script setup lang="ts">
definePageMeta({ layout: "auth" });

const {
  step,
  form,
  errors,
  loading,
  submitError,
  handleRequestCode,
  handleSetPassword,
} = useResetPassword();
</script>

<template>
  <div class="flex flex-col flex-1">
    <AppPageHeader
      v-if="step === 'email'"
      title="Reset your password"
      description="Enter your account email and we'll send you a reset code."
    />
    <AppPageHeader
      v-else
      title="Set a new password"
      description="Choose a new password for your account."
    />

    <form
      class="flex flex-col gap-4"
      @submit.prevent="
        step === 'email' ? handleRequestCode() : handleSetPassword()
      "
    >
      <template v-if="step === 'email'">
        <AppInput
          v-model="form.email"
          type="email"
          name="email"
          label="Email"
          placeholder="you@example.com"
          size="lg"
          :disabled="loading"
        />
        <p v-if="errors.email" class="text-xs text-red-500 -mt-3">
          {{ errors.email }}
        </p>

        <p v-if="submitError" class="text-sm text-red-500">{{ submitError }}</p>

        <AppButton type="submit" :loading="loading" :disabled="loading">
          Send reset code
        </AppButton>
      </template>

      <template v-else>
        <AppInput
          v-model="form.password"
          type="password"
          name="password"
          label="New password"
          placeholder="At least 6 characters"
          size="lg"
          :disabled="loading"
        />
        <p v-if="errors.password" class="text-xs text-red-500 -mt-3">
          {{ errors.password }}
        </p>

        <AppInput
          v-model="form.confirmPassword"
          type="password"
          name="confirmPassword"
          label="Confirm new password"
          placeholder="Repeat your new password"
          size="lg"
          :disabled="loading"
        />
        <p v-if="errors.confirmPassword" class="text-xs text-red-500 -mt-3">
          {{ errors.confirmPassword }}
        </p>

        <p v-if="submitError" class="text-sm text-red-500">{{ submitError }}</p>

        <AppButton
          type="submit"
          color="primary"
          size="xl"
          class="w-full py-3 bg-primary-500 hover:bg-primary-600 rounded-lg text-white font-medium disabled:opacity-50 text-center"
          :loading="loading"
          :disabled="loading"
        >
          Update password
        </AppButton>
      </template>
    </form>

    <p class="text-sm text-left text-gray-500 dark:text-gray-400 pt-4">
      Remembered your password?
      <NuxtLink
        to="/auth/login"
        class="text-primary-500 hover:text-primary-600 font-medium"
      >
        Sign in
      </NuxtLink>
    </p>
  </div>
</template>
