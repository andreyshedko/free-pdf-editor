import { PDFDocument } from 'pdf-lib';

/**
 * Given raw PDF bytes, returns saved PDF bytes.
 * Note: password encryption is not yet implemented; the password parameter is reserved for future use.
 */
export async function exportPdf(sourceBytes: ArrayBuffer, _password?: string): Promise<Uint8Array> {
  const pdfDoc = await PDFDocument.load(sourceBytes, { ignoreEncryption: true });
  const saveOptions: Parameters<typeof pdfDoc.save>[0] = {};
  return pdfDoc.save(saveOptions);
}

/**
 * Reorder/delete pages by given order array.
 */
export async function reorderPages(sourceBytes: ArrayBuffer, pageOrder: number[]): Promise<Uint8Array> {
  const srcDoc = await PDFDocument.load(sourceBytes, { ignoreEncryption: true });
  const newDoc = await PDFDocument.create();
  const pages = await newDoc.copyPages(srcDoc, pageOrder);
  pages.forEach((page) => newDoc.addPage(page));
  return newDoc.save();
}

/**
 * Embed a signature (PNG data URL) onto the given page in the bottom-right corner.
 */
export async function embedSignature(
  sourceBytes: ArrayBuffer,
  pageIndex: number,
  signatureDataUrl: string
): Promise<Uint8Array> {
  const pdfDoc = await PDFDocument.load(sourceBytes, { ignoreEncryption: true });
  const page = pdfDoc.getPage(pageIndex);
  const { width, height } = page.getSize();
  const base64 = signatureDataUrl.split(',')[1];
  const pngBytes = Uint8Array.from(atob(base64), (c) => c.charCodeAt(0));
  const img = await pdfDoc.embedPng(pngBytes);
  const sigWidth = Math.min(200, width * 0.4);
  const sigHeight = sigWidth * 0.5;
  page.drawImage(img, {
    x: width - sigWidth - 20,
    y: 20,
    width: sigWidth,
    height: sigHeight,
  });
  return pdfDoc.save();
}
