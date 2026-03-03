import { describe, it, expect, vi, beforeEach } from 'vitest';

const { mockPage, mockDoc } = vi.hoisted(() => {
  const mockPage = {
    getSize: vi.fn().mockReturnValue({ width: 595, height: 842 }),
    drawText: vi.fn(),
  };
  const mockDoc = {
    embedFont: vi.fn().mockResolvedValue({}),
    getPage: vi.fn().mockReturnValue(mockPage),
    save: vi.fn().mockResolvedValue(new Uint8Array([1, 2, 3])),
  };
  return { mockPage, mockDoc };
});

vi.mock('pdf-lib', () => ({
  PDFDocument: {
    load: vi.fn().mockResolvedValue(mockDoc),
  },
  StandardFonts: { Helvetica: 'Helvetica' },
  rgb: vi.fn((r: number, g: number, b: number) => ({ r, g, b })),
}));

import { PDFDocument, rgb } from 'pdf-lib';
import { insertText } from './editorService';

describe('editorService', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    (PDFDocument.load as ReturnType<typeof vi.fn>).mockResolvedValue(mockDoc);
    mockDoc.embedFont.mockResolvedValue({});
    mockDoc.getPage.mockReturnValue(mockPage);
    mockDoc.save.mockResolvedValue(new Uint8Array([1, 2, 3]));
    mockPage.getSize.mockReturnValue({ width: 595, height: 842 });
    mockPage.drawText.mockReset();
  });

  describe('insertText', () => {
    const insertion = {
      text: 'Hello',
      pageIndex: 0,
      x: 50,
      y: 100,
      fontSize: 14,
      color: '#ff0000',
    };

    it('loads the PDF with ignoreEncryption: true', async () => {
      const buf = new ArrayBuffer(8);
      await insertText(buf, insertion);
      expect(PDFDocument.load).toHaveBeenCalledWith(buf, { ignoreEncryption: true });
    });

    it('retrieves the correct page', async () => {
      await insertText(new ArrayBuffer(8), insertion);
      expect(mockDoc.getPage).toHaveBeenCalledWith(insertion.pageIndex);
    });

    it('draws text with converted y coordinate (top-left to bottom-left origin)', async () => {
      await insertText(new ArrayBuffer(8), insertion);
      expect(mockPage.drawText).toHaveBeenCalledWith(
        insertion.text,
        expect.objectContaining({
          x: insertion.x,
          y: 842 - insertion.y - insertion.fontSize,
        }),
      );
    });

    it('converts hex color to rgb components', async () => {
      await insertText(new ArrayBuffer(8), { ...insertion, color: '#ff0000' });
      expect(rgb).toHaveBeenCalledWith(1, 0, 0);
    });

    it('returns a Uint8Array', async () => {
      const result = await insertText(new ArrayBuffer(8), insertion);
      expect(result).toBeInstanceOf(Uint8Array);
    });
  });
});
