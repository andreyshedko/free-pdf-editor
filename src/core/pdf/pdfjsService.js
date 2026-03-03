import * as pdfjsLib from 'pdfjs-dist';
import pdfWorkerUrl from 'pdfjs-dist/build/pdf.worker.min.mjs?url';

pdfjsLib.GlobalWorkerOptions.workerSrc = pdfWorkerUrl;

let cached = null;
let cachedSource = null;

export async function getOrLoadPdfDocument(data) {
  if (cached && cachedSource === data) {
    return cached;
  }

  const loadingTask = pdfjsLib.getDocument({ data });
  const doc = await loadingTask.promise;
  cached = doc;
  cachedSource = data;
  return doc;
}
