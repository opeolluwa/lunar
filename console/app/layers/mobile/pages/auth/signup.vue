<script setup lang="ts">
definePageMeta({ layout: "auth" });

const { form, errors, loading, submitError, handleSubmit } = useSignup();
</script>

<template>
  <div class="flex flex-col flex-1">
    <AppPageHeader
      title="Create your account"
      description="Get started in a few seconds."
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

      <AppInput
        v-model="form.password"
        type="password"
        name="password"
        label="Password"
        placeholder="At least 6 characters"
        size="lg"
        :disabled="loading"
        :enable-password-toggle="false"
      />
      <p v-if="errors.password" class="text-xs text-red-500 -mt-3">
        {{ errors.password }}
      </p>

      <AppInput
        v-model="form.confirmPassword"
        type="password"
        name="confirmPassword"
        label="Confirm password"
        placeholder="Repeat your password"
        size="xl"
        :disabled="loading"
        :enable-password-toggle="false"
      />
      <p v-if="errors.confirmPassword" class="text-xs text-red-500 -mt-3">
        {{ errors.confirmPassword }}
      </p>

      <p v-if="submitError" class="text-sm text-red-500">{{ submitError }}</p>

      <AppButton type="submit" :loading="loading" :disabled="loading">
        Create account
      </AppButton>
    </form>

    <p class="text-sm text-left text-gray-500 dark:text-gray-400 pt-4">
      Already have an account?
      <NuxtLink
        to="/auth/login"
        class="text-primary-500 hover:text-primary-600 font-medium"
      >
        Sign in
      </NuxtLink>
    </p>
  </div>
</template>
