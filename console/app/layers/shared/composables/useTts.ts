import { speak, stop, onSpeechEvent } from "tauri-plugin-tts-api";

export function plainTextFromHtml(html: string): string {
  return html
    .replace(/<br\s*\/?>/gi, " ")
    .replace(/<\/(p|div|h[1-6]|li|tr|pre|blockquote|table)>/gi, " ")
    .replace(/<[^>]+>/g, "")
    .replace(/&nbsp;/g, " ")
    .replace(/&amp;/g, "&")
    .replace(/&lt;/g, "<")
    .replace(/&gt;/g, ">")
    .replace(/&quot;/g, '"')
    .replace(/&#39;|&apos;/g, "'")
    .replace(/\s+/g, " ")
    .trim();
}

let subscribed = false;

export function useTTS() {
  const isSpeaking = ref(false);

  async function subscribe() {
    if (subscribed) return;
    subscribed = true;
    for (const eventType of [
      "speech:start",
      "speech:finish",
      "speech:cancel",
      "speech:interrupted",
      "speech:error",
    ] as const) {
      await onSpeechEvent(eventType, () => {
        isSpeaking.value = eventType === "speech:start";
      });
    }
  }

  async function speakText(text: string) {
    if (!text || isSpeaking.value) return;
    if (isTauri()) {
      try {
        await subscribe();
      } catch (e) {
        console.error("[tts] failed to subscribe to speech events", e);
      }
    }
    isSpeaking.value = true;
    try {
      await speak({ text });
    } catch (e) {
      isSpeaking.value = false;
      console.error("[tts] failed to speak", e);
    }
  }

  async function stopSpeaking() {
    try {
      await stop();
    } finally {
      isSpeaking.value = false;
    }
  }

  return { isSpeaking, speakText, stopSpeaking };
}

function isTauri() {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}