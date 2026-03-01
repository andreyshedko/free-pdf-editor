import { describe, it, expect, vi, beforeEach } from 'vitest';

// pdf-lib performs canvas operations that are unavailable in the test environment.
// Provide a minimal mock so the module can be imported without a DOM.
vi.mock('pdf-lib', () => {
  const mockPage = {};
  const mockDoc = {
    save: vi.fn().mockResolvedValue(new Uint8Array([1, 2, 3])),
    addPage: vi.fn(),
    copyPages: vi.fn().mockResolvedValue([mockPage]),
  };
  return {
    PDFDocument: {
      load: vi.fn().mockResolvedValue(mockDoc),
      create: vi.fn().mockResolvedValue(mockDoc),
    },
  };
});

import { exportPdf, reorderPages } from './exportService';

describe('exportService', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('exportPdf returns a Uint8Array', async () => {
    const buf = new ArrayBuffer(8);
    const result = await exportPdf(buf);
    expect(result).toBeInstanceOf(Uint8Array);
  });

  it('reorderPages returns a Uint8Array', async () => {
    const buf = new ArrayBuffer(8);
    const result = await reorderPages(buf, [0, 1]);
    expect(result).toBeInstanceOf(Uint8Array);
  });
});
