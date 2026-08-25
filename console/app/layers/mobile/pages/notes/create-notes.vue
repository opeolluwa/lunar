<script setup lang="ts">
import { kFab } from "konsta/vue";
import { useNoteStore } from "@shared/stores/notes";
import { onBeforeRouteLeave } from "vue-router";
import EditorToolBar from "@mobile/components/notes/EditorToolBar.vue";
import NoteTitleInput from "@shared/components/notes/note-title-input.vue";
definePageMeta({ name: "New note", layout: "notes", keepalive: true });

const router = useRouter();
const noteStore = useNoteStore();

const title = ref("");
const content = ref("");
const submitting = ref(false);
const saved = ref(false);
const error = ref<string | null>(null);

onActivated(() => {
  title.value = "";
  content.value = "";
  error.value = null;
  submitting.value = false;
  saved.value = false;
});

const lastSaved = ref<Date | null>(null);

const hasContent = computed(
  () => !!title.value.trim() || !!content.value.trim(),
);

async function handleSave() {
  if (!hasContent.value) return;
  submitting.value = true;
  error.value = null;
  try {
    await noteStore.createNote({
      title: title.value.trim() || "Untitled",
      content: content.value,
    });
    saved.value = true;
    lastSaved.value = new Date();
    router.push("/notes");
  } catch (e) {
    error.value = String(e);
    submitting.value = false;
  }
}

useEventListener("keydown", (e: KeyboardEvent) => {
  if ((e.metaKey || e.ctrlKey) && e.key === "s") {
    e.preventDefault();
    if (hasContent.value && !submitting.value) handleSave();
  }
});

onBeforeRouteLeave(async () => {
  if (submitting.value || saved.value) return;
  if (!hasContent.value) return;
  try {
    await noteStore.createNote({
      title: title.value.trim() || "Untitled",
      content: content.value,
    });
  } catch (e) {
    console.error(e);
  }
});
</script>

<template>
  <div class="pb-24">
    <NoteTitleInput v-model="title" :disabled="submitting" />

    <NotesEditor v-model="content">
      <template #toolbar>
        <EditorToolBar />
      </template>
    </NotesEditor>

    <p v-if="error" class="text-xs text-red-500 mt-6">
      {{ error }}
    </p>

    <kFab
      component="button"
      aria-label="Save note"
      class="absolute right-7 md:hidden"
      :style="
        'bottom: calc(var(--kb-inset, 0px) + env(safe-area-inset-bottom) + 4.5rem);'
      "
      :colors="{
        bgIos: 'bg-primary-500 dark:bg-primary-600',
        bgMaterial: 'bg-primary-500 dark:bg-primary-600',
        textIos: 'text-white',
        textMaterial: 'text-white',
      }"
      @click="handleSave"
    >
      <template #icon>
        <UIcon name="ri:save-line" class="size-6" />
      </template>
    </kFab>
  </div>
</template>
