<script setup lang="ts">
import { useUserPreferenceStore } from "@shared/stores/workspace-profile";

definePageMeta({ name: "Profile" });

const store = useUserPreferenceStore();

const form = reactive({
  firstName: store.preference?.firstName ?? "",
  lastName: store.preference?.lastName ?? "",
  email: store.preference?.email ?? "",
});

watch(
  () => store.preference,
  (pref) => {
    if (pref) {
      form.firstName = pref.firstName;
      form.lastName = pref.lastName;
      form.email = pref.email;
    }
  },
);

const saving = ref(false);

async function handleSave() {
  saving.value = true;

  try {
    await store.updatePreference({
      firstName: form.firstName.trim(),
      lastName: form.lastName.trim(),
      email: form.email.trim(),
    });
  } finally {
    saving.value = false;
  }
}
</script>

<template>
  <div>
    <UUser
      name="John Doe"
      class="flex justify-center items-center"
      description="Software Engineer"
      :avatar="{
        src: 'https://i.pravatar.cc/150?u=john-doe',
        loading: 'lazy',
        icon: 'i-lucide-image',
      }"
      :ui="{
        avatar: 'w-24 h-24',
        wrapper: 'flex justify-center items-center flex-col',
      }"
      orientation="vertical"
    />

    <form class="mt-6 flex flex-col gap-y-5">
      <AppInput
        v-model="form.firstName"
        type="text"
        placeholder="John"
        label="First name"
        name="first-name"
      />

      <AppInput
        v-model="form.lastName"
        type="text"
        placeholder="Doe"
        label="Last name"
        name="last-name"
      />

      <AppInput
        v-model="form.email"
        type="email"
        placeholder="john@example.com"
        label="Email"
        name="email"
      />

      <AppButton :disabled="saving" class="justify-center" @click="handleSave">
        {{ saving ? "Saving…" : "Save changes" }}
      </AppButton>
    </form>
  </div>
</template>
