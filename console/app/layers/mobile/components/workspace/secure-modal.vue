<script setup lang="ts">
import { useWorkspacesStore } from "@shared/stores/workspaces";
import AppInput from "@shared/components/app/Input.vue";

const props = defineProps<{
  open: boolean;
  workspaceId: string | null;
}>();

const emit = defineEmits<{
  "update:open": [value: boolean];
}>();

const { notify } = useAppNotification();
const workspaceStore = useWorkspacesStore();

const password = ref("");
const confirm = ref("");
const error = ref("");
const submitting = ref(false);

const targetWorkspace = computed(() =>
  props.workspaceId
    ? workspaceStore.workspaces?.find((w) => w.identifier === props.workspaceId)
    : null,
);

function close() {
  emit("update:open", false);
}

async function handleSubmit() {
  if (!props.workspaceId || !targetWorkspace.value) return;

  if (!targetWorkspace.value.isSecured) {
    if (!password.value) {
      error.value = "Password is required.";
      return;
    }
    if (password.value !== confirm.value) {
      error.value = "Passwords do not match.";
      return;
    }
  }

  submitting.value = true;
  error.value = "";
  try {
    if (targetWorkspace.value.isSecured) {
      const ok = await workspaceStore.verifyWorkspacePassword(
        props.workspaceId,
        password.value,
      );
      if (!ok) {
        error.value = "Incorrect password.";
        return;
      }
      await workspaceStore.updateWorkspace(props.workspaceId, {
        isSecured: false,
        password: "",
      });
      notify({ message: "Workspace password removed", type: "success" });
    } else {
      await workspaceStore.updateWorkspace(props.workspaceId, {
        isSecured: true,
        password: password.value,
      });
      workspaceStore.unlockWorkspace(props.workspaceId);
      notify({ message: "Workspace secured with password", type: "success" });
    }
    close();
  } catch (e) {
    notify({
      message: (e as Error).message || "Failed to update workspace security",
      type: "error",
    });
  } finally {
    submitting.value = false;
  }
}
</script>

<template>
  <UModal :open="open" @close="close">
    <template #content>
      <div class="p-6 flex flex-col gap-4">
        <h3 class="text-sm font-semibold text-gray-800 dark:text-gray-200">
          {{
            targetWorkspace?.isSecured
              ? "Remove workspace password"
              : "Set workspace password"
          }}
        </h3>
        <p class="text-xs text-gray-500 dark:text-gray-400">
          {{
            targetWorkspace?.isSecured
              ? "Enter the current password to remove protection from this workspace."
              : "This workspace will require a password before it can be accessed."
          }}
        </p>
        <UFormField
          :label="targetWorkspace?.isSecured ? 'Current password' : 'Password'"
        >
          <AppInput
            v-model="password"
            type="password"
            placeholder="Enter password"
            class="w-full"
            :disabled="submitting"
          />
        </UFormField>
        <UFormField v-if="!targetWorkspace?.isSecured" label="Confirm password">
          <AppInput
            v-model="confirm"
            type="password"
            placeholder="Confirm password"
            class="w-full"
            :disabled="submitting"
          />
        </UFormField>
        <p v-if="error" class="text-xs text-red-500 dark:text-red-400">
          {{ error }}
        </p>
        <div class="flex items-center gap-2 mt-2">
          <UButton
            variant="ghost"
            size="sm"
            :disabled="submitting"
            @click="close"
          >
            Cancel
          </UButton>

          <UButton
            size="sm"
            :color="targetWorkspace?.isSecured ? 'error' : 'primary'"
            :loading="submitting"
            :disabled="!password"
            @click="handleSubmit"
          >
            {{
              targetWorkspace?.isSecured ? "Remove password" : "Set password"
            }}
          </UButton>
        </div>
      </div>
    </template>
  </UModal>
</template>
