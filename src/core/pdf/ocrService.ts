import { createWorker } from 'tesseract.js';

let workerPromise: Promise<Awaited<ReturnType<typeof createWorker>>> | null = null;

function getWorker() {
  if (!workerPromise) {
    workerPromise = createWorker('eng');
  }
  return workerPromise;
}

/**
 * Run OCR on a canvas element and return recognized text.
 */
export async function recognizeCanvas(canvas: HTMLCanvasElement): Promise<string> {
  const worker = await getWorker();
  const { data: { text } } = await worker.recognize(canvas);
  return text;
}

/**
 * Terminate the background Tesseract worker and release its memory.
 * Call this when OCR is no longer needed (e.g. on app unload).
 */
export async function terminateWorker(): Promise<void> {
  if (workerPromise) {
    const worker = await workerPromise;
    await worker.terminate();
    workerPromise = null;
  }
}
