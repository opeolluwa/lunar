import { useAuthStore } from "@shared/stores/auth";
import type { MaybeRefOrGetter } from "vue";
import type { LoginRequest } from "lunar";

export function useLogin(
  prefillEmail?: MaybeRefOrGetter<string>,
  options: { rememberEmail?: boolean } = {},
) {
  const authApi = useAuthApi();
  const authStore = useAuthStore();
  const { notify } = useAppNotification();
  const emailMemory = options.rememberEmail ? useRememberedEmail() : null;

  const form = reactive<LoginRequest>({
    email: prefillEmail ? toValue(prefillEmail) : "",
    password: "",
  });
  const errors = reactive({ email: "", password: "" });
  const loading = ref(false);
  const submitError = ref("");

  function validate(): boolean {
    errors.email = emailValidator(form.email) ? "" : "A valid email is required";
    errors.password = form.password ? "" : "Password is required";
    return !errors.email && !errors.password;
  }

  async function handleSubmit() {
    if (!validate()) return;
    loading.value = true;
    submitError.value = "";
    try {
      const response = await authApi.login({
        email: form.email.trim(),
        password: form.password,
      });
      authStore.setSession(
        response.accessToken,
        response.refreshToken,
        response.exp,
      );
      emailMemory?.rememberEmail(form.email.trim());
      notify({ message: "Logged in successfully", type: "success" });
      await navigateTo("/");
    } catch (error) {
      submitError.value = (error as Error).message;
    } finally {
      loading.value = false;
    }
  }

  function continueWithoutLogin() {
    authStore.enterGuestMode();
    navigateTo("/");
  }

  return {
    form,
    errors,
    loading,
    submitError,
    handleSubmit,
    continueWithoutLogin,
  };
}