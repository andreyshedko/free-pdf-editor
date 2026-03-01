import { describe, it, expect, vi, beforeEach } from 'vitest';

// vi.hoisted runs before imports, making the mock objects available inside vi.mock().
const { mockSrcDoc, mockNewDoc } = vi.hoisted(() => {
  const mockSrcDoc = {
    save: vi.fn().mockResolvedValue(new Uint8Array([1, 2, 3])),
    embedPng: vi.fn().mockResolvedValue({}),
    getPage: vi.fn().mockReturnValue({
      getSize: vi.fn().mockReturnValue({ width: 595, height: 842 }),
      drawImage: vi.fn(),
    }),
  };
  const mockNewDoc = {
    save: vi.fn().mockResolvedValue(new Uint8Array([4, 5, 6])),
    addPage: vi.fn(),
    copyPages: vi.fn().mockImplementation(
      (_src: unknown, indices: number[]) => Promise.resolve(indices.map(() => ({}))),
    ),
  };
  return { mockSrcDoc, mockNewDoc };
});

vi.mock('pdf-lib', () => ({
  PDFDocument: {
    load: vi.fn().mockResolvedValue(mockSrcDoc),
    create: vi.fn().mockResolvedValue(mockNewDoc),
  },
}));

import { PDFDocument } from 'pdf-lib';
import { exportPdf, reorderPages, embedSignature } from './exportService';

describe('exportService', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    // Re-apply mock implementations cleared by clearAllMocks
    (PDFDocument.load as ReturnType<typeof vi.fn>).mockResolvedValue(mockSrcDoc);
    (PDFDocument.create as ReturnType<typeof vi.fn>).mockResolvedValue(mockNewDoc);
    mockSrcDoc.save.mockResolvedValue(new Uint8Array([1, 2, 3]));
    mockSrcDoc.embedPng.mockResolvedValue({});
    mockSrcDoc.getPage.mockReturnValue({
      getSize: vi.fn().mockReturnValue({ width: 595, height: 842 }),
      drawImage: vi.fn(),
    });
    mockNewDoc.save.mockResolvedValue(new Uint8Array([4, 5, 6]));
    mockNewDoc.copyPages.mockImplementation(
      (_src: unknown, indices: number[]) => Promise.resolve(indices.map(() => ({}))),
    );
  });

  describe('exportPdf', () => {
    it('loads the source PDF with ignoreEncryption: true', async () => {
      const buf = new ArrayBuffer(8);
      await exportPdf(buf);
      expect(PDFDocument.load).toHaveBeenCalledWith(buf, { ignoreEncryption: true });
    });

    it('returns a Uint8Array', async () => {
      const result = await exportPdf(new ArrayBuffer(8));
      expect(result).toBeInstanceOf(Uint8Array);
    });
  });

  describe('reorderPages', () => {
    it('loads the source PDF with ignoreEncryption: true', async () => {
      const buf = new ArrayBuffer(8);
      await reorderPages(buf, [0, 1]);
      expect(PDFDocument.load).toHaveBeenCalledWith(buf, { ignoreEncryption: true });
    });

    it('copies pages in the requested order', async () => {
      const order = [2, 0, 1];
      await reorderPages(new ArrayBuffer(8), order);
      expect(mockNewDoc.copyPages).toHaveBeenCalledWith(mockSrcDoc, order);
    });

    it('adds one page per copied page', async () => {
      const order = [0, 1, 2];
      await reorderPages(new ArrayBuffer(8), order);
      expect(mockNewDoc.addPage).toHaveBeenCalledTimes(order.length);
    });

    it('returns a Uint8Array', async () => {
      const result = await reorderPages(new ArrayBuffer(8), [0, 1]);
      expect(result).toBeInstanceOf(Uint8Array);
    });
  });

  describe('embedSignature', () => {
    // 1×1 transparent PNG as base64
    const pngDataUrl =
      'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==';

    it('loads the source PDF with ignoreEncryption: true', async () => {
      const buf = new ArrayBuffer(8);
      await embedSignature(buf, 0, pngDataUrl);
      expect(PDFDocument.load).toHaveBeenCalledWith(buf, { ignoreEncryption: true });
    });

    it('embeds the PNG and draws it on the requested page', async () => {
      const mockPage = { getSize: vi.fn().mockReturnValue({ width: 595, height: 842 }), drawImage: vi.fn() };
      mockSrcDoc.getPage.mockReturnValue(mockPage);
      await embedSignature(new ArrayBuffer(8), 0, pngDataUrl);
      expect(mockSrcDoc.embedPng).toHaveBeenCalled();
      expect(mockPage.drawImage).toHaveBeenCalled();
    });

    it('returns a Uint8Array', async () => {
      const result = await embedSignature(new ArrayBuffer(8), 0, pngDataUrl);
      expect(result).toBeInstanceOf(Uint8Array);
    });
  });
});
