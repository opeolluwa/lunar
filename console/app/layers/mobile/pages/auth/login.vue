<script setup lang="ts">
definePageMeta({ layout: "auth" });

const { form, errors, loading, submitError, handleSubmit } = useLogin(
  undefined,
  { rememberEmail: true },
);
</script>

<template>
  <div class="flex flex-col flex-1">
    <AppPageHeader
      title="Welcome back"
      description="Sign in to your Lunar account to continue."
    />

    <form @submit.prevent="handleSubmit">
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

      <div>
        <AppInput
          v-model="form.password"
          type="password"
          name="password"
          label="Password"
          placeholder="••••••••"
          size="lg"
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

      <AppButton
        type="submit"
        class="text-center align-center"
        :loading="loading"
        :disabled="loading"
        size="xl"
      >
        Sign in
      </AppButton>
    </form>

    <p
      class="text-sm text-left text-gray-500 dark:text-gray-400 w-full left-0 pt-4"
    >
      Don't have an account?
      <NuxtLink
        to="/auth/signup"
        class="text-primary-500 hover:text-primary-600 font-medium"
      >
        Sign up
      </NuxtLink>
    </p>
  </div>
</template>
