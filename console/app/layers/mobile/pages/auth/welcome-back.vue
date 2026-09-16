<script setup lang="ts">
definePageMeta({ layout: "auth" });

const { rememberedEmail } = useRememberedEmail();
const { form, errors, loading, submitError, handleSubmit } = useLogin(
  rememberedEmail,
);

function handleLogin() {
  if (!rememberedEmail.value) {
    navigateTo("/auth/login");
    return;
  }
  handleSubmit();
}
</script>

<template>
  <div class="flex flex-col flex-1">
    <AppPageHeader
      title="Welcome back, Adeoye"
      description="Enter your password to continue."
    />
    <form @submit.prevent="handleLogin">
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
  </div>
</template>
