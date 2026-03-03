import { create } from 'zustand';

export type ViewMode = 'fit-width' | 'fit-page' | 'custom';

export interface PdfDocumentState {
  fileData: ArrayBuffer | null;
  currentFileName: string | null;
  pageCount: number | null;
  currentPageIndex: number;
  zoom: number;
  viewMode: ViewMode;
  setDocument: (opts: { data: ArrayBuffer; name: string }) => void;
  updateFileData: (data: ArrayBuffer) => void;
  setPageCount: (count: number) => void;
  setCurrentPageIndex: (index: number) => void;
  setZoom: (zoom: number) => void;
  setViewMode: (mode: ViewMode) => void;
}

export const usePdfDocumentStore = create<PdfDocumentState>((set) => ({
  fileData: null,
  currentFileName: null,
  pageCount: null,
  currentPageIndex: -1,
  zoom: 100,
  viewMode: 'fit-width',
  setDocument: ({ data, name }) =>
    set({
      fileData: data,
      currentFileName: name,
      pageCount: null,
      currentPageIndex: 0,
      zoom: 100,
      viewMode: 'fit-width'
    }),
  updateFileData: (data: ArrayBuffer) => set({ fileData: data, pageCount: null }),
  setPageCount: (count: number) =>
    set((state) => ({
      pageCount: state.pageCount ?? count
    })),
  setCurrentPageIndex: (index: number) =>
    set((state) => ({
      currentPageIndex: Math.max(0, Math.min(index, (state.pageCount ?? 1) - 1))
    })),
  setZoom: (zoom: number) =>
    set({ zoom: Math.max(25, Math.min(200, zoom)), viewMode: 'custom' }),
  setViewMode: (viewMode: ViewMode) => set({ viewMode })
}));

