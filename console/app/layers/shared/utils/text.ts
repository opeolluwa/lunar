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

export const MAX_CHUNK_LENGTH = 160;

export function splitTextIntoChunks(text: string): string[] {
  const chunks: string[] = [];
  const sentences = text.match(/[^.!?]+[.!?]+\s*|[^.!?]+$/g) ?? [];
  let buffer = "";

  for (const sentence of sentences) {
    const word = sentence.trim();
    if (!word) continue;

    if (word.length > MAX_CHUNK_LENGTH) {
      if (buffer) chunks.push(buffer.trim());
      let rest = word;
      while (rest.length > MAX_CHUNK_LENGTH) {
        chunks.push(rest.slice(0, MAX_CHUNK_LENGTH));
        rest = rest.slice(MAX_CHUNK_LENGTH);
      }
      buffer = rest;
      continue;
    }

    if ((buffer + " " + word).length > MAX_CHUNK_LENGTH) {
      chunks.push(buffer.trim());
      buffer = word;
    } else {
      buffer = buffer ? buffer + " " + word : word;
    }
  }

  if (buffer) chunks.push(buffer.trim());
  return chunks.filter(Boolean);
}
