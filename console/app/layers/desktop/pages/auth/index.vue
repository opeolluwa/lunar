<script setup lang="ts">
import { useAuthStore } from "@shared/stores/auth";

definePageMeta({ layout: false });

const authStore = useAuthStore();
const { form, errors, loading, submitError, handleSubmit, continueWithoutLogin } =
  useLogin();

onMounted(() => {
  if (authStore.isAuthenticated || authStore.isGuest) {
    navigateTo("/");
  }
});
</script>

<template>
  <NuxtLayout name="auth">
    <div class="flex flex-col gap-5">
      <AppPageHeader
        title="Welcome back"
        description="Sign in to your Lunar account to continue."
      />

      <form class="flex flex-col gap-4" @submit.prevent="handleSubmit">
        <AppInput
          v-model="form.email"
          type="email"
          name="email"
          label="Email"
          hint="required"
          placeholder="you@example.com"
          :disabled="loading"
        />
        <p v-if="errors.email" class="text-xs text-red-500 -mt-3">
          {{ errors.email }}
        </p>

        <div>
          <AppInput
            v-model="form.password"
            type="password"
            name="password"
            label="Password"
            hint="required"
            placeholder="••••••••"
            :disabled="loading"
          />
          <div class="flex justify-end mt-1">
            <NuxtLink
              to="/auth/reset-password"
              class="text-xs text-primary-500 hover:text-primary-600 font-medium"
            >
              Forgot password?
            </NuxtLink>
          </div>
        </div>
        <p v-if="errors.password" class="text-xs text-red-500 -mt-3">
          {{ errors.password }}
        </p>

        <p v-if="submitError" class="text-sm text-red-500">{{ submitError }}</p>

        <AppButton type="submit" :loading="loading" :disabled="loading">
          Sign in
        </AppButton>
      </form>

      <p class="text-sm text-center text-gray-500 dark:text-gray-400">
        Don't have an account?
        <NuxtLink
          to="/auth/signup"
          class="text-primary-500 hover:text-primary-600 font-medium"
        >
          Sign up
        </NuxtLink>
      </p>
    </div>

    <template #below-card>
      <div class="flex flex-col items-center gap-2 mt-4">
        <NuxtLink
          type="link"
          color="neutral"
          class="px-6 py-2.5 rounded-lg text-sm text-gray-600 dark:text-gray-400 dark:hover:bg-gray-700 cursor-pointer"
          :disabled="loading"
          @click="continueWithoutLogin"
        >
          <p class="text-sm text-gray-400 dark:text-gray-500">
            Continue without account
          </p>
        </NuxtLink>
      </div>
    </template>
  </NuxtLayout>
</template>
