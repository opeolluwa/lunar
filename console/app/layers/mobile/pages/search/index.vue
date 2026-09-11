<script setup lang="ts">
import { useReminderStore } from "@shared/stores/reminder";
import { useSnippetStore } from "@shared/stores/snippets";
import { useNoteStore } from "@shared/stores/notes";
import { useBookmarkStore } from "@shared/stores/bookmarks";
import { useTodoStore } from "@shared/stores/todo";
import { safeOpenUrl as openUrl } from "@shared/utils/safe-open-url";

definePageMeta({ layout: "search", name: "Search" });

const router = useRouter();
const { searchQuery, clearSearch } = useAppSearch();

const noteStore = useNoteStore();
const bookmarkStore = useBookmarkStore();
const snippetStore = useSnippetStore();
const todoStore = useTodoStore();
const reminderStore = useReminderStore();

onMounted(async () => {
  await Promise.all(
    [
      noteStore.notes.length === 0 ? noteStore.fetchNotes() : null,
      bookmarkStore.bookmarks.length === 0
        ? bookmarkStore.fetchBookmarks()
        : null,
      snippetStore.snippets.length === 0 ? snippetStore.fetchSnippets() : null,
      todoStore.todos.length === 0 ? todoStore.fetchTodos() : null,
      reminderStore.reminders.length === 0
        ? reminderStore.fetchReminders()
        : null,
    ].filter(Boolean),
  );
});

onUnmounted(() => clearSearch());

const typeConfig = {
  note: {
    icon: "heroicons:document-text",
    color: "text-violet-500 dark:text-violet-400",
    bg: "bg-violet-50 dark:bg-violet-950/60",
    label: "Notes",
  },
  bookmark: {
    icon: "heroicons:bookmark-solid",
    color: "text-primary-500 dark:text-primary-400",
    bg: "bg-primary-50 dark:bg-primary-950/60",
    label: "Bookmarks",
  },
  snippet: {
    icon: "heroicons:code-bracket",
    color: "text-blue-500 dark:text-blue-400",
    bg: "bg-blue-50 dark:bg-blue-950/60",
    label: "Snippets",
  },
  todo: {
    icon: "heroicons:check-circle",
    color: "text-emerald-500 dark:text-emerald-400",
    bg: "bg-emerald-50 dark:bg-emerald-950/60",
    label: "Todos",
  },
  reminder: {
    icon: "heroicons:bell",
    color: "text-amber-500 dark:text-amber-400",
    bg: "bg-amber-50 dark:bg-amber-950/60",
    label: "Reminders",
  },
} as const;

type ResultType = keyof typeof typeConfig;

interface SearchResult {
  id: string;
  type: ResultType;
  title: string;
  sub: string;
  href: string;
  externalUrl?: string;
}

const sections = computed(() => {
  const q = searchQuery.value.trim().toLowerCase();
  if (!q) return [];

  const out: { label: string; type: ResultType; items: SearchResult[] }[] = [];

  const notes = noteStore.notes
    .filter(
      (n) =>
        n.title?.toLowerCase().includes(q) ||
        n.content?.toLowerCase().includes(q),
    )
    .slice(0, 4)
    .map<SearchResult>((n) => ({
      id: n.identifier,
      type: "note",
      title: n.title || "Untitled",
      sub: n.categories?.length ? n.categories.join(", ") : "Note",
      href: `/notes/edit-notes?id=${n.identifier}`,
    }));
  if (notes.length) out.push({ label: "Notes", type: "note", items: notes });

  const bookmarks = bookmarkStore.bookmarks
    .filter(
      (b) =>
        b.title?.toLowerCase().includes(q) ||
        b.url?.toLowerCase().includes(q) ||
        b.tag?.toLowerCase().includes(q),
    )
    .slice(0, 4)
    .map<SearchResult>((b) => ({
      id: b.identifier,
      type: "bookmark",
      title: b.title,
      sub: b.url,
      href: "/bookmarks",
      externalUrl: b.url,
    }));
  if (bookmarks.length)
    out.push({ label: "Bookmarks", type: "bookmark", items: bookmarks });

  const snippets = snippetStore.snippets
    .filter(
      (s) =>
        s.title?.toLowerCase().includes(q) ||
        s.language?.toLowerCase().includes(q) ||
        s.description?.toLowerCase().includes(q) ||
        s.code?.toLowerCase().includes(q),
    )
    .slice(0, 4)
    .map<SearchResult>((s) => ({
      id: s.identifier,
      type: "snippet",
      title: s.title || "Untitled",
      sub: s.language || "snippet",
      href: `/snippets/view-snippet?id=${s.identifier}`,
    }));
  if (snippets.length)
    out.push({ label: "Snippets", type: "snippet", items: snippets });

  const todos = todoStore.todos
    .filter(
      (t) =>
        t.title?.toLowerCase().includes(q) ||
        t.description?.toLowerCase().includes(q),
    )
    .slice(0, 4)
    .map<SearchResult>((t) => ({
      id: t.identifier,
      type: "todo",
      title: t.title,
      sub: t.priority,
      href: `/todo/edit-todo?id=${t.identifier}`,
    }));
  if (todos.length) out.push({ label: "Todos", type: "todo", items: todos });

  const reminders = reminderStore.reminders
    .filter(
      (r) =>
        r.title?.toLowerCase().includes(q) ||
        r.description?.toLowerCase().includes(q),
    )
    .slice(0, 4)
    .map<SearchResult>((r) => ({
      id: r.identifier,
      type: "reminder",
      title: r.title,
      sub: new Date(r.remindAt).toLocaleDateString("en-US", {
        month: "short",
        day: "numeric",
        year: "numeric",
      }),
      href: `/reminders/edit-reminder?id=${r.identifier}`,
    }));
  if (reminders.length)
    out.push({ label: "Reminders", type: "reminder", items: reminders });

  return out;
});

const hasQuery = computed(() => searchQuery.value.trim().length > 0);
const hasResults = computed(() =>
  sections.value.some((s) => s.items.length > 0),
);

function navigate(item: SearchResult) {
  if (item.externalUrl) {
    openUrl(item.externalUrl);
    clearSearch();
  } else {
    router.push(item.href);
  }
}
</script>

<template>
  <div class="flex flex-col px-6 pb-10">
    <div
      v-if="!hasQuery"
      class="flex flex-col items-center px-8 py-24 text-center"
    >
      <UIcon
        name="heroicons:magnifying-glass"
        class="size-12 text-gray-300 dark:text-gray-600"
      />
      <p class="mt-4 text-sm font-medium text-gray-500 dark:text-gray-400">
        Search everything
      </p>
      <p class="mt-1 text-xs leading-relaxed text-gray-400 dark:text-gray-500">
        Find notes, bookmarks, snippets, todos and reminders across your
        workspace.
      </p>
    </div>

    <div
      v-else-if="!hasResults"
      class="flex flex-col items-center px-8 py-24 text-center"
    >
      <UIcon
        name="heroicons:no-symbol"
        class="size-12 text-gray-300 dark:text-gray-600"
      />
      <p class="mt-4 text-sm font-medium text-gray-500 dark:text-gray-400">
        No results for "{{ searchQuery }}"
      </p>
      <p class="mt-1 text-xs text-gray-400 dark:text-gray-500">
        Try a different search term.
      </p>
    </div>

    <template v-else>
      <div v-for="section in sections" :key="section.type" class="mb-3">
        <div class="flex items-center gap-2 px-1 pb-1 pt-5">
          <span
            class="inline-flex size-4 items-center justify-center rounded"
            :class="typeConfig[section.type].bg"
          >
            <UIcon
              :name="typeConfig[section.type].icon"
              class="size-2.5"
              :class="typeConfig[section.type].color"
            />
          </span>
          <span
            class="text-[11px] font-semibold uppercase tracking-wider text-gray-400 dark:text-gray-500"
          >
            {{ section.label }}
          </span>
        </div>

        <div
          class="overflow-hidden rounded-2xl border border-gray-100 bg-white dark:border-gray-700 dark:bg-gray-800/60"
        >
          <button
            v-for="(item, index) in section.items"
            :key="item.id"
            class="flex w-full items-center gap-3 px-4 py-3.5 text-left transition-colors active:bg-gray-50 dark:active:bg-white/5"
            :class="
              index !== section.items.length - 1
                ? 'border-b border-gray-100 dark:border-gray-700/60'
                : ''
            "
            @click="navigate(item)"
          >
            <span
              class="inline-flex size-8 shrink-0 items-center justify-center rounded-lg"
              :class="typeConfig[item.type].bg"
            >
              <UIcon
                :name="typeConfig[item.type].icon"
                class="size-4"
                :class="typeConfig[item.type].color"
              />
            </span>
            <span class="min-w-0 flex-1">
              <span
                class="block truncate text-sm font-medium text-gray-800 dark:text-gray-100"
              >
                {{ item.title }}
              </span>
              <span
                class="mt-0.5 block truncate text-xs text-gray-400 dark:text-gray-500"
              >
                {{ item.sub }}
              </span>
            </span>
            <UIcon
              name="heroicons:chevron-right"
              class="size-4 shrink-0 text-gray-300 dark:text-gray-600"
            />
          </button>
        </div>
      </div>
    </template>
  </div>
</template>
