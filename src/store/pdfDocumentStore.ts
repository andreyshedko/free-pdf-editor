import { create } from 'zustand';

export interface PdfDocumentState {
  fileData: ArrayBuffer | null;
  currentFileName: string | null;
  pageCount: number | null;
  currentPageIndex: number;
  setDocument: (opts: { data: ArrayBuffer; name: string }) => void;
  setPageCount: (count: number) => void;
  setCurrentPageIndex: (index: number) => void;
}

export const usePdfDocumentStore = create<PdfDocumentState>((set) => ({
  fileData: null,
  currentFileName: null,
  pageCount: null,
  currentPageIndex: -1,
  setDocument: ({ data, name }) =>
    set({
      fileData: data,
      currentFileName: name,
      pageCount: null,
      currentPageIndex: 0
    }),
  setPageCount: (count: number) =>
    set((state) => ({
      pageCount: state.pageCount ?? count
    })),
  setCurrentPageIndex: (index: number) =>
    set((state) => ({
      currentPageIndex: Math.max(0, Math.min(index, (state.pageCount ?? 1) - 1))
    }))
}));

