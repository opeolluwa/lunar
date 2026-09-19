<script lang="ts" setup>
import { useNoteStore } from "@shared/stores/notes";
import { plainTextFromHtml } from "@shared/utils/text";
import { speak, stop } from "tauri-plugin-tts-api";

const route = useRoute();
const noteStore = useNoteStore();

const isPlaying = ref(false);
const text = ref("");

const playIcon = "ri:play-circle-fill";
const pauseIcon = "ri:pause-circle-fill";

const currentIcon = computed(() => (isPlaying.value ? pauseIcon : playIcon));

const noteId = computed(() => route.query.id as string);

const loadNoteText = () => {
  const note = noteStore.getNoteById(noteId.value);

  text.value = note?.content ? plainTextFromHtml(note.content) : "";
};

const synthText = async () => {
  if (isPlaying.value) {
    await stop();
    isPlaying.value = false;
    return;
  }

  if (!text.value) {
    return;
  }

  isPlaying.value = true;

  try {
    await speak({
      text: text.value,
    });
  } finally {
    isPlaying.value = false;
  }
};

onMounted(loadNoteText);

watch(noteId, loadNoteText);
</script>

<template>
  <div class="flex">
    <UIcon
      :name="currentIcon"
      class="size-5 cursor-pointer"
      @click="synthText"
    />
  </div>
</template>
