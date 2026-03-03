import { PDFDocument, StandardFonts, rgb } from 'pdf-lib';

function hexToRgb(hex) {
  const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
  return result
    ? {
        r: parseInt(result[1], 16) / 255,
        g: parseInt(result[2], 16) / 255,
        b: parseInt(result[3], 16) / 255,
      }
    : { r: 0, g: 0, b: 0 };
}

/**
 * Insert text onto the given page. x/y coordinates are measured from the
 * top-left corner (screen convention); they are converted internally to the
 * PDF coordinate system (origin at bottom-left, y increases upward).
 */
export async function insertText(sourceBytes, insertion) {
  const pdfDoc = await PDFDocument.load(sourceBytes, { ignoreEncryption: true });
  const font = await pdfDoc.embedFont(StandardFonts.Helvetica);
  const page = pdfDoc.getPage(insertion.pageIndex);
  const { height: pageHeight } = page.getSize();
  const { r, g, b } = hexToRgb(insertion.color);
  page.drawText(insertion.text, {
    x: insertion.x,
    y: pageHeight - insertion.y - insertion.fontSize,
    size: insertion.fontSize,
    font,
    color: rgb(r, g, b),
  });
  return pdfDoc.save();
}
