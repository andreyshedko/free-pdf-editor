import * as pdfjsLib from 'pdfjs-dist';

// Worker URL for PDF.js - Vite resolves this at build time
import pdfWorkerUrl from 'pdfjs-dist/build/pdf.worker.min.mjs?url';

pdfjsLib.GlobalWorkerOptions.workerSrc = pdfWorkerUrl;

let cached: pdfjsLib.PDFDocumentProxy | null = null;
let cachedSource: ArrayBuffer | null = null;

export async function getOrLoadPdfDocument(data: ArrayBuffer): Promise<pdfjsLib.PDFDocumentProxy> {
  if (cached && cachedSource === data) {
    return cached;
  }

  const loadingTask = pdfjsLib.getDocument({ data });
  const doc = await loadingTask.promise;
  cached = doc;
  cachedSource = data;
  return doc;
}

