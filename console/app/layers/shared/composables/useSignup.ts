import { useAuthStore } from "@shared/stores/auth";

export function useSignup() {
  const authApi = useAuthApi();
  const authStore = useAuthStore();
  const { notify } = useAppNotification();

  const form = reactive({ email: "", password: "", confirmPassword: "" });
  const errors = reactive({ email: "", password: "", confirmPassword: "" });
  const loading = ref(false);
  const submitError = ref("");

  function validate(): boolean {
    errors.email = emailValidator(form.email) ? "" : "A valid email is required";
    errors.password =
      form.password.length >= 6 ? "" : "Password must be at least 6 characters";
    errors.confirmPassword =
      form.confirmPassword === form.password ? "" : "Passwords do not match";
    return !errors.email && !errors.password && !errors.confirmPassword;
  }

  async function handleSubmit() {
    if (!validate()) return;
    loading.value = true;
    submitError.value = "";
    try {
      const response = await authApi.signup({
        email: form.email.trim(),
        password: form.password,
      });
      authStore.setPendingToken(response.token);
      notify({
        message: "Account created. Check your email for a verification code.",
        type: "success",
        duration: 5000,
      });
      await navigateTo(`/auth/confirm-otp?flow=verify`);
    } catch (error) {
      submitError.value = (error as Error).message;
    } finally {
      loading.value = false;
    }
  }

  return { form, errors, loading, submitError, handleSubmit };
}