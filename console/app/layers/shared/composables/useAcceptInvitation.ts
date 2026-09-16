export function useAcceptInvitation() {
  const route = useRoute();
  const authApi = useAuthApi();
  const { notify } = useAppNotification();

  const token = computed(() => (route.query.token as string | undefined) ?? "");
  const email = computed(() => (route.query.email as string | undefined) ?? "");

  const loading = ref(false);
  const submitError = ref("");

  async function handleAccept() {
    loading.value = true;
    submitError.value = "";
    try {
      await authApi.acceptInvitation({ token: token.value });
      notify({
        message: "Invitation accepted. Welcome aboard!",
        type: "success",
      });
      await navigateTo("/");
    } catch (error) {
      submitError.value = (error as Error).message;
    } finally {
      loading.value = false;
    }
  }

  return { token, email, loading, submitError, handleAccept };
}