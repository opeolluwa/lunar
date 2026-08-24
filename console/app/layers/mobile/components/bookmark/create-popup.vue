<script setup lang="ts">
import { kPage, kNavbar, kPopup, kBlock } from "konsta/vue";
import {
  useBookmarkStore,
  type Bookmark,
  type BookmarkTag,
} from "@shared/stores/bookmarks";
import AppButton from "@shared/components/app/Button.vue";

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
  <kPopup :opened="open" @backdropclick="requestClose">
    <kPage>
      <kNavbar
        :title="title"
        bg-class="bg-white dark:bg-app-dark-800"
        class="px-3"
      >
        <template #right>
          <UButton
            size="md"
            color="neutral"
            variant="ghost"
            icon="heroicons:x-mark"
            class="text-gray-400 dark:text-gray-500"
            aria-label="Close"
            :disabled="loading"
            @click="requestClose"
          />
        </template>
      </kNavbar>

      <kBlock strong inset class="space-y-4">
        <p class="mb-2 text-sm text-gray-500 dark:text-gray-400">
          {{ description }}
        </p>

        <form class="flex flex-col gap-4" @submit.prevent="handleSubmit">
          <AppInput
            v-model="form.title"
            label="Title"
            hint="required"
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
            hint="required"
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
            <div class="flex flex-wrap gap-1.5">
              <button
                v-for="tag in TAGS"
                :key="tag.value"
                type="button"
                class="px-3 py-1.5 rounded-lg text-xs font-medium transition-colors disabled:opacity-50"
                :class="
                  form.tag === tag.value
                    ? 'bg-primary-50 dark:bg-primary-950 text-primary-700 dark:text-primary-300 ring-1 ring-primary-200 dark:ring-primary-800'
                    : 'bg-gray-100 dark:bg-gray-700 text-gray-500 dark:text-gray-400'
                "
                :disabled="loading"
                @click="form.tag = tag.value"
              >
                {{ tag.label }}
              </button>
            </div>
          </div>

          <p v-if="submitError" class="text-sm text-red-500">
            {{ submitError }}
          </p>

          <div class="flex gap-2 pt-2">
            <AppButton
              type="button"
              variant="outline"
              size="sm"
              class="flex-1 justify-center"
              :disabled="loading"
              @click="requestClose"
            >
              Cancel
            </AppButton>
            <AppButton
              type="submit"
              size="sm"
              class="flex-1 justify-center"
              :loading="loading"
            >
              {{ submitLabel }}
            </AppButton>
          </div>
        </form>
      </kBlock>
    </kPage>
  </kPopup>
</template>
