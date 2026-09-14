<script setup lang="ts">
import EmptyState from "@shared/components/app/EmptyState.vue";
import NotesCard from "@mobile/components/notes/card.vue";
import { sortNotes, type NoteSort } from "@shared/utils/sorting";
import { useNoteStore } from "@shared/stores/notes";
const noteStore = useNoteStore();
const { searchQuery, clearSearch } = useAppSearch();
const sortBy = ref<NoteSort>("date-newest");

definePageMeta({
  layout: "notes",
  name: "Notes",
});

onMounted(async () => {
  await noteStore.fetchNotes();
});

onUnmounted(() => clearSearch());

const filteredNotes = computed(() => {
  const q = searchQuery.value.trim().toLowerCase();
  const list = q
    ? noteStore.notes.filter(
        (n) =>
          n.title.toLowerCase().includes(q) ||
          n.content.toLowerCase().includes(q),
      )
    : noteStore.notes;

  return sortNotes(list, sortBy.value);
});
</script>

<template>
  <AppPullToRefresh @refresh="() => noteStore.fetchNotes()">
    <div>
      <!-- Create note FAB -->
      <AppFab
        v-if="!noteStore.loading && filteredNotes.length > 0"
        @click="navigateTo('/notes/create-notes')"
      />

      <!-- Loading -->
      <div v-if="noteStore.loading" class="flex flex-col gap-3">
        <USkeleton v-for="i in 4" :key="i" class="h-24 rounded-lg" />
      </div>

      <template v-else>
        <!-- Empty state: no notes at all -->
        <div v-if="noteStore.notes.length === 0">
          <EmptyState
            title="No notes yet"
            description="Create your first note to get started."
            icon="ri:booklet-line"
            action-label="create note"
            @action="navigateTo('/notes/create-notes')"
          />
        </div>

        <template v-else>
          <!-- Empty state: search has no results -->
          <EmptyState
            v-if="filteredNotes.length === 0"
            title="No results found"
            description="Try a different search term."
            icon="heroicons:magnifying-glass"
            action-label="clear search"
            @action="searchQuery = ''"
          />

          <!-- Notes list -->
          <div v-else class="flex flex-col gap-3">
            <NotesCard
              v-for="note in filteredNotes"
              :key="note.identifier"
              :identifier="note.identifier"
              :title="note.title"
              :content="note.content"
              :updated-at="note.updatedAt"
            />
          </div>
        </template>
      </template>
    </div>
  </AppPullToRefresh>
</template>
