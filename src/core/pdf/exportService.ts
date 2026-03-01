import { PDFDocument } from 'pdf-lib';

/**
 * Given raw PDF bytes, returns saved PDF bytes.
 * Note: password encryption is not yet implemented; the password parameter is reserved for future use.
 */
export async function exportPdf(sourceBytes: ArrayBuffer, password?: string): Promise<Uint8Array> {
  const pdfDoc = await PDFDocument.load(sourceBytes, { ignoreEncryption: true });
  const saveOptions: Parameters<typeof pdfDoc.save>[0] = {};
  // password parameter is accepted for future encryption support
  void password;
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
