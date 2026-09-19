import { speak, stop, onSpeechEvent } from "tauri-plugin-tts-api";


function webSynth(): SpeechSynthesis | null {
  return typeof window !== "undefined" && "speechSynthesis" in window
    ? window.speechSynthesis
    : null;
}

let subscribed = false;

export function useTTS() {
  const isSpeaking = ref(false);

  let webChunks: string[] = [];
  let webIndex = 0;
  let webCancelled = false;

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

  function nextWebChunk() {
    const synth = webSynth();
    if (!synth || webCancelled) return;
    if (webIndex >= webChunks.length) {
      isSpeaking.value = false;
      return;
    }

    const utterance = new SpeechSynthesisUtterance(webChunks[webIndex]);
    utterance.onstart = () => {
      isSpeaking.value = true;
    };
    utterance.onend = () => {
      webIndex += 1;
      nextWebChunk();
    };
    utterance.onerror = (event) => {
      if (webCancelled) return;
      console.error("[tts] speech synthesis error", event.error);
      isSpeaking.value = false;
      webChunks = [];
    };
    synth.speak(utterance);
  }

  function speakWeb(text: string) {
    const synth = webSynth();
    if (!synth) {
      console.error("[tts] SpeechSynthesis is not supported in this browser");
      return;
    }

    const chunks = splitIntoChunks(text);
    if (chunks.length === 0) return;

    synth.cancel();
    webChunks = chunks;
    webIndex = 0;
    webCancelled = false;
    isSpeaking.value = true;
    setTimeout(nextWebChunk, 0);
  }

  async function speakText(text: string) {
    if (!text || isSpeaking.value) return;

    if (isTauri()) {
      try {
        await subscribe();
      } catch (e) {
        console.error("[tts] failed to subscribe to speech events", e);
      }
      isSpeaking.value = true;
      try {
        await speak({ text });
      } catch (e) {
        isSpeaking.value = false;
        console.error("[tts] failed to speak", e);
      }
      return;
    }

    speakWeb(text);
  }

  async function stopSpeaking() {
    if (isTauri()) {
      try {
        await stop();
      } finally {
        isSpeaking.value = false;
      }
      return;
    }

    webCancelled = true;
    const synth = webSynth();
    if (synth) synth.cancel();
    webChunks = [];
    webIndex = 0;
    isSpeaking.value = false;
  }

  return { isSpeaking, speakText, stopSpeaking };
}

function isTauri() {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}