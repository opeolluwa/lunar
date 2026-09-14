<script setup lang="ts">
import { marked } from "marked";
import DOMPurify from "dompurify";
import katex from "katex";
import { Domternal } from "@domternal/vue";
import type { Editor } from "@domternal/core";
import { Extension,
  StarterKit,
  BubbleMenu,
  BaseKeymap,
  Superscript,
  Subscript,
  Text,
  TextStyle,
  TextAlign,
  TextColor,
  Code,
  Heading,
  ListItem,
  BulletList,
  OrderedList,
  Link,
  Placeholder,
  UniqueID,
  BlockColor,
  ListIndent,
  Print } from "@domternal/core";
import { Plugin, PluginKey } from "@domternal/pm/state";
import { DOMParser as PMDOMParser } from "@domternal/pm/model";
import { Details } from "@domternal/extension-details";
import { CodeBlockLowlight } from "@domternal/extension-code-block-lowlight";
import { createLowlight, all } from "lowlight";
import { Table } from "@domternal/extension-table";
import { Image } from "@domternal/extension-image";
import {
  MathInline,
  MathBlock,
  createKatexRenderer,
} from "@domternal/extension-math";
import "katex/dist/katex.min.css";

import {
  Emoji,
  emojis,
  createEmojiSuggestionRenderer,
} from "@domternal/extension-emoji";
import {
  BlockContextMenu,
  BlockHandle,
  KeyboardReorder,
  SlashCommand,
  SmartPaste,
} from "@domternal/extension-block-controls";

const colorMode = useColorMode();
const isDark = computed(() => colorMode.value === "dark");
const lowlight = createLowlight(all);
const mathRenderer = createKatexRenderer(katex);
const dmVars = computed(() =>
  isDark.value
    ? {
        "--dm-primary": "var(--color-primary-400)",
        "--dm-primary-hover": "var(--color-primary-300)",
        "--dm-primary-surface":
          "color-mix(in srgb, var(--color-primary-400) 15%, transparent)",
        "--dm-bg": "var(--color-surface-900)",
        "--dm-surface": "var(--color-surface-800)",
        "--dm-border-color": "var(--color-surface-700)",
      }
    : {
        "--dm-primary": "var(--color-primary-500)",
        "--dm-primary-hover": "var(--color-primary-600)",
        "--dm-primary-surface":
          "color-mix(in srgb, var(--color-primary-500) 10%, transparent)",
        "--dm-block-handle-gutter": 0,
      },
);

// When raw markdown is pasted (plain text with no HTML counterpart),
// convert it to rich content through the editor's schema.
const MarkdownPaste = Extension.create({
  name: "markdownPaste",

  addProseMirrorPlugins() {
    return [
      new Plugin({
        key: new PluginKey("markdownPaste"),
        props: {
          handlePaste(view, event, _slice) {
            const clipboard = event.clipboardData;
            if (!clipboard) return false;

            // Pasting from a website/browser includes text/html; let PM default handle those.
            if (clipboard.getData("text/html")) return false;

            const text = clipboard.getData("text/plain");
            if (!text || !isLikelyMarkdown(text)) return false;

            const mdHtml = marked.parse(text) as string;
            const wrapper = document.createElement("div");
            wrapper.innerHTML = mdHtml;
            const parsed = PMDOMParser.fromSchema(view.state.schema).parse(
              wrapper,
            );

            const tr = view.state.tr.replaceSelectionWith(parsed);
            view.dispatch(tr.scrollIntoView());
            return true;
          },
        },
      }),
    ];
  },
});

// eslint-disable-next-line @typescript-eslint/no-explicit-any
function handleUpdate({ editor }: { editor: any }) {
  model.value = DOMPurify.sanitize(editor.getHTML());
}

const extensions = [
  StarterKit,
  BubbleMenu,
  UniqueID,
  BlockColor,
  ListIndent,
  Print,
  Table,
  ListItem.configure({
    HTMLAttributes: { class: "notes_list_item" },
  }),
  BulletList.configure({
    HTMLAttributes: { class: "note_list_unordered" },
  }),
  OrderedList.configure({
    HTMLAttributes: { class: "note_list_ordered" },
  }),
  Superscript,
  Subscript,
  Text,
  BaseKeymap,
  Details,
  TextStyle,
  Code,
  TextAlign,
  TextColor,
  BlockHandle.configure({ nested: true }),
  BlockContextMenu,
  SlashCommand,
  SmartPaste,
  MarkdownPaste,
  KeyboardReorder,
  Heading.configure({
    levels: [1, 2, 3, 4, 5, 6],
    HTMLAttributes: { class: "notes_heading" },
  }),
  CodeBlockLowlight.configure({ lowlight }),
  MathInline.configure({ renderer: mathRenderer }),
  MathBlock.configure({ renderer: mathRenderer }),
  Emoji.configure({
    emojis,
    suggestion: { render: createEmojiSuggestionRenderer() },
  }),
  Link.configure({
    protocols: ["http:", "https:"],
    openOnClick: true,
    autolink: true,
    linkOnPaste: true,
    defaultProtocol: "https",
  }),
  Image.configure({
    //TODO: replace with actual upload handler that uploads to server and returns URL
    uploadHandler: async (file) => {
      const form = new FormData();
      form.append("file", file);
      const res = await fetch("/api/upload", { method: "POST", body: form });
      const { url } = await res.json();
      return url;
    },
  }),

  Placeholder.configure({
    placeholder: ({ node }) => {
      if (node.type.name === "heading") return "Enter a heading...";
      if (node.type.name === "codeBlock") return "// Write code here";
      if (node.type.name === "table") return "";
      return "Type something...";
    },
  }),
];

const model = defineModel<string>();

const editor = shallowRef<Editor | null>(null);

function handleCreate(ed: Editor) {
  editor.value = ed;
}

function handleDestroy() {
  editor.value = null;
}

defineExpose({
  editor,
});

// Old notes were saved as Markdown (UEditor content-type="markdown").
// Domternal expects HTML, so detect and convert on the way in.
function isHtml(s: string): boolean {
  const t = s.trimStart();
  return t.startsWith("<") && /<[a-z][\s\S]*>/i.test(t);
}

const initialContent = computed(() => {
  const raw = model.value ?? "";
  if (!raw || isHtml(raw)) return raw;
  return marked.parse(raw) as string;
});

// Quick heuristic to detect raw markdown (vs plain prose) in pasted text.
function isLikelyMarkdown(text: string): boolean {
  const t = text.trim();
  if (!t) return false;
  const patterns = [
    /^#{1,6}\s/m, // ATX headings
    /\*\*[^*]+\*\*/, // **bold**
    /_[^_]+_/, // _italic_
    /`[^`]+`/, // inline code
    /```/, // fenced code blocks
    /^\s{4}.*$/m, // indented code blocks
    /^\s*[-*+]\s/m, // unordered lists
    /^\s*\d+\.\s/m, // ordered lists
    /\[[^[\]]*\]\([^)]+\)/, // [text](url)
    /!\[[^[\]]*]\([^)]+\)/, // ![alt](url)
    /^>\s/m, // blockquotes
    /^---+$/m, // horizontal rule
    /^[-*_]\s*[-*_]\s*[-*_]/, // horizontal rule (variants)
  ];
  return patterns.some((p) => p.test(text));
}
</script>

<template>
  <div
    :class="{ 'dm-theme-dark': isDark }"
    :style="dmVars"
    class="notes-editor"
  >
    <Domternal
      :extensions="extensions"
      :content="initialContent"
      :on-update="handleUpdate"
      :on-create="handleCreate"
      :on-destroy="handleDestroy"
    >
      <Domternal.Content class="bg-transparent -ml-12" />
      <Domternal.BubbleMenu class="mb-5" />
      <slot name="toolbar" />
    </Domternal>
  </div>
</template>

<style>
.dm-editor {
  --dm-editor-bg: transparent;
  --dm-editor-border-radius: 0;
  --dm-editor-shadow: none;
  --dm-editor-border: none;
  --dm-border-color: #cccccc;
  --dm-code-surface: transparent;
}

.notes-editor .dm-editor .ProseMirror {
  --dm-editor-padding: 0;
  --dm-editor-padding-top-extra: 0.25rem;
}
</style>
