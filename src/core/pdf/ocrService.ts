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
