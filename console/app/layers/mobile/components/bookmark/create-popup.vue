<script setup lang="ts">
import { kSheet } from "konsta/vue";
import {
  useBookmarkStore,
  type Bookmark,
  type BookmarkTag,
} from "@shared/stores/bookmarks";

withDefaults(
  defineProps<{
    open: boolean;
    title?: string;
    description?: string;
    submitLabel?: string;
  }>(),
  {
    title: "New Bookmark",
    description: "Save links you want to keep track of.",
    submitLabel: "Save bookmark",
  },
);

const emit = defineEmits<{
  "update:open": [value: boolean];
  created: [bookmark: Bookmark];
}>();

const bookmarkStore = useBookmarkStore();

const TAGS: { label: string; value: BookmarkTag }[] = [
  { label: "Development", value: "development" },
  { label: "Design", value: "design" },
  { label: "Research", value: "research" },
  { label: "Inspiration", value: "inspiration" },
];

const form = reactive({
  title: "",
  url: "",
  tag: "development" as BookmarkTag,
});
const errors = reactive({ title: "", url: "" });
const loading = ref(false);
const submitError = ref("");

function resetForm() {
  Object.assign(form, {
    title: "",
    url: "",
    tag: "development",
  });
  Object.assign(errors, { title: "", url: "" });
  submitError.value = "";
}

function requestClose() {
  resetForm();
  emit("update:open", false);
}

async function handleSubmit() {
  errors.title = form.title.trim() ? "" : "Title is required";
  errors.url = form.url.trim() ? "" : "URL is required";
  if (errors.title || errors.url) return;
  loading.value = true;
  submitError.value = "";
  try {
    const created = await bookmarkStore.createBookmark({
      title: form.title.trim(),
      url: form.url.trim(),
      tag: form.tag,
    });
    resetForm();
    emit("update:open", false);
    emit("created", created);
  } catch (e) {
    console.error(e);
    submitError.value = "Failed to save bookmark. Please try again.";
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <kSheet :opened="open" @backdropclick="requestClose">
    <div
      class="flex max-h-[85dvh] flex-col overflow-hidden rounded-t-[20px] bg-white pb-1 dark:bg-app-dark-800"
      style="padding-bottom: env(safe-area-inset-bottom, 0px)"
    >
      <div
        class="mx-auto my-3 h-1 w-9 shrink-0 rounded-full bg-gray-300 dark:bg-gray-600"
      />

      <div class="shrink-0 px-4 pb-5">
        <div class="flex items-center justify-between">
          <h2 class="text-lg font-semibold leading-6 text-gray-900 dark:text-white">
            {{ title }}
          </h2>
          <UButton
            size="md"
            color="neutral"
            variant="ghost"
            icon="heroicons:x-mark"
            aria-label="Close"
            :disabled="loading"
            class="-mr-2"
            @click="requestClose"
          />
        </div>
        <p class="pt-1 text-sm text-gray-500 dark:text-gray-400">
          {{ description }}
        </p>
      </div>

      <form
        class="flex min-h-0 flex-1 flex-col"
        @submit.prevent="handleSubmit"
      >
        <div
          class="flex-1 overflow-y-auto px-4 pb-4"
          style="-webkit-overflow-scrolling: touch"
        >
          <div class="flex flex-col gap-4">
            <AppInput
              v-model="form.title"
              label="Title"
              type="text"
              name="bookmark-title"
              placeholder="Bookmark title"
              :disabled="loading"
            />
            <p v-if="errors.title" class="-mt-3 text-xs text-red-500">
              {{ errors.title }}
            </p>

            <AppInput
              v-model="form.url"
              label="URL"
              type="text"
              name="bookmark-url"
              placeholder="https://example.com"
              :disabled="loading"
            />
            <p v-if="errors.url" class="-mt-3 text-xs text-red-500">
              {{ errors.url }}
            </p>

            <div class="flex flex-col gap-1.5">
              <label class="text-xs font-medium text-gray-600 dark:text-gray-400">
                Tag
              </label>
              <URadioGroup
                v-model="form.tag"
                :items="TAGS"
                variant="card"
                orientation="horizontal"
                size="sm"
                :disabled="loading"
              />
            </div>

            <p v-if="submitError" class="text-sm text-red-500">
              {{ submitError }}
            </p>
          </div>
        </div>

        <div
          class="flex shrink-0 items-center justify-end gap-3 px-4 pb-2 pt-3"
        >
          <UButton type="submit" :loading="loading">
            {{ submitLabel }}
          </UButton>
        </div>
      </form>
    </div>
  </kSheet>
</template>
