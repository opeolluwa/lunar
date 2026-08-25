<script setup lang="ts">
import { kPage, kNavbar, kPopup, kBlock } from "konsta/vue";
import { useTodoStore } from "@shared/stores/todo";
import type { Todo } from "@shared/stores/todo";

withDefaults(
  defineProps<{
    open: boolean;
    title?: string;
    description?: string;
    submitLabel?: string;
  }>(),
  {
    title: "New Task",
    description: "Tasks help you keep track of what needs to get done.",
    submitLabel: "Create task",
  },
);

const emit = defineEmits<{
  "update:open": [value: boolean];
  created: [todo: Todo];
}>();

const todoStore = useTodoStore();

const priorityOptions = [
  { label: "Low", value: "low" },
  { label: "Medium", value: "medium" },
  { label: "High", value: "high" },
];

const form = reactive({
  title: "",
  description: "",
  dueDate: null as Date | null,
  priority: "medium" as "high" | "medium" | "low",
});
const selectedTime = shallowRef();
const errors = reactive({ title: "" });
const loading = ref(false);
const submitError = ref("");

function toIsoDate(date: Date | null): string | null {
  if (!date) return null;
  return [
    date.getFullYear(),
    String(date.getMonth() + 1).padStart(2, "0"),
    String(date.getDate()).padStart(2, "0"),
  ].join("-");
}

function formatDisplayDate(date: Date | null): string {
  if (!date) return "";
  return date.toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
  });
}

function resetForm() {
  Object.assign(form, {
    title: "",
    description: "",
    dueDate: null,
    priority: "medium",
  });
  Object.assign(errors, { title: "" });
  selectedTime.value = undefined;
  submitError.value = "";
}

function requestClose() {
  resetForm();
  emit("update:open", false);
}

async function handleSubmit() {
  errors.title = form.title.trim() ? "" : "Title is required";
  if (errors.title) return;
  loading.value = true;
  submitError.value = "";
  try {
    const created = await todoStore.createTodo({
      title: form.title.trim(),
      description: form.description.trim() || undefined,
      dueDate: toIsoDate(form.dueDate) ?? undefined,
      time: selectedTime.value
        ? `${String(selectedTime.value.hour).padStart(2, "0")}:${String(selectedTime.value.minute).padStart(2, "0")}`
        : undefined,
      priority: form.priority,
    });
    resetForm();
    emit("update:open", false);
    emit("created", created);
  } catch (e) {
    console.error(e);
    submitError.value = "Failed to create task. Please try again.";
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <kPopup :opened="open" @backdropclick="requestClose">
    <kPage class="bg-gray-50 dark:bg-app-dark-800">
      <kNavbar bg-class="bg-white dark:bg-app-dark-800" class="px-3">
        <template #title>
          <AppPageTitle>{{ title }}</AppPageTitle>
        </template>
        <template #right>
          <UButton
            size="md"
            color="neutral"
            variant="ghost"
            icon="heroicons:x-mark"
            aria-label="Close"
            :disabled="loading"
            @click="requestClose"
          />
        </template>
      </kNavbar>

      <kBlock inset class="mx-3 mt-6">
        <p class="mb-4 mt-3 text-sm text-gray-500 dark:text-gray-400">
          {{ description }}
        </p>

        <form class="flex flex-col gap-4" @submit.prevent="handleSubmit">
          <AppInput
            v-model="form.title"
            label="Title"
            type="text"
            name="todo-title"
            placeholder="What needs to be done?"
            :disabled="loading"
          />
          <p v-if="errors.title" class="-mt-3 text-xs text-red-500">
            {{ errors.title }}
          </p>

          <div class="flex flex-col gap-1.5">
            <label class="text-xs font-medium text-gray-600 dark:text-gray-400">
              Description
            </label>
            <textarea
              v-model="form.description"
              placeholder="Add more details..."
              rows="3"
              class="almond_input_box resize-none"
              :disabled="loading"
            />
          </div>

          <div class="flex flex-col gap-1.5">
            <label class="text-xs font-medium text-gray-600 dark:text-gray-400">
              Due date
            </label>
            <div class="flex items-center gap-2">
              <UPopover class="flex-1">
                <button
                  type="button"
                  class="w-full flex items-center gap-2 bg-white dark:bg-gray-800 rounded-lg px-4 py-2.5 text-sm border border-gray-200 dark:border-gray-700 hover:border-gray-300 dark:hover:border-gray-600 transition-colors text-left disabled:opacity-50"
                  :class="
                    form.dueDate
                      ? 'text-gray-700 dark:text-gray-200'
                      : 'text-gray-400 dark:text-gray-500'
                  "
                  :disabled="loading"
                >
                  <UIcon
                    name="heroicons:calendar"
                    class="size-4 shrink-0 text-gray-400"
                  />
                  {{
                    form.dueDate
                      ? formatDisplayDate(form.dueDate)
                      : "Pick a date"
                  }}
                </button>
                <template #content="{ close }">
                  <AppDatePicker v-model="form.dueDate" @update:model-value="close" />
                </template>
              </UPopover>
              <button
                v-if="form.dueDate"
                type="button"
                class="p-2.5 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 transition-colors"
                aria-label="Clear due date"
                @click="form.dueDate = null"
              >
                <UIcon name="heroicons:x-mark" class="size-4" />
              </button>
            </div>
          </div>

          <div class="flex flex-col gap-1.5">
            <label class="text-xs font-medium text-gray-600 dark:text-gray-400">
              Time
            </label>
            <div class="flex items-center gap-2">
              <UInputTime
                v-model="selectedTime"
                icon="i-lucide-clock"
                class="flex-1"
                :disabled="loading"
              />
              <button
                v-if="selectedTime"
                type="button"
                class="p-2.5 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 transition-colors"
                aria-label="Clear time"
                @click="selectedTime = undefined"
              >
                <UIcon name="heroicons:x-mark" class="size-4" />
              </button>
            </div>
          </div>

          <div class="flex flex-col gap-1.5">
            <label class="text-xs font-medium text-gray-600 dark:text-gray-400">
              Priority
            </label>
            <URadioGroup
              v-model="form.priority"
              :items="priorityOptions"
              orientation="horizontal"
              size="sm"
              :disabled="loading"
            />
          </div>

          <p v-if="submitError" class="text-sm text-red-500">
            {{ submitError }}
          </p>

          <div class="flex gap-2 pt-2 justify-between">
            <UButton
              color="error"
              variant="outline"
              :disabled="loading"
              @click="requestClose"
            >
              Cancel
            </UButton>
            <UButton type="submit" :loading="loading">
              {{ submitLabel }}
            </UButton>
          </div>
        </form>
      </kBlock>
    </kPage>
  </kPopup>
</template>
