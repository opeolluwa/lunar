<script lang="ts" setup>
import { useNoteStore } from '@shared/stores/notes';
import { useTTS, plainTextFromHtml } from '@shared/composables/useTts';

const noteStore = useNoteStore();
const { isSpeaking, speakText, stopSpeaking } = useTTS();

const noteHtml = computed(() => noteStore.currentNoteHtml);

function togglePlay() {
  if (isSpeaking.value) {
    stopSpeaking();
    return;
  }

  const text = plainTextFromHtml(noteHtml.value);
  if (!text) return;

  speakText(text);
}

const playIcon = computed(() =>
  isSpeaking.value ? "lucide:circle-stop" : "lucide:circle-play",
);
</script>
<template>
  <div class="flex">
    <button type="button" aria-label="Play note" @click="togglePlay">
      <UIcon :name="playIcon" class="size-5" />
    </button>
  </div>
</template>