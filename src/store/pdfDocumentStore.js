import { create } from 'zustand';

export const usePdfDocumentStore = create((set) => ({
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
  updateFileData: (data) => set({ fileData: data, pageCount: null }),
  setPageCount: (count) =>
    set((state) => ({
      pageCount: state.pageCount ?? count
    })),
  setCurrentPageIndex: (index) =>
    set((state) => ({
      currentPageIndex: Math.max(0, Math.min(index, (state.pageCount ?? 1) - 1))
    })),
  setZoom: (zoom) =>
    set({ zoom: Math.max(25, Math.min(200, zoom)), viewMode: 'custom' }),
  setViewMode: (viewMode) => set({ viewMode })
}));
