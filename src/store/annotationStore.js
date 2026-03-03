import { create } from 'zustand';

export const useAnnotationStore = create((set) => ({
  activeTool: null,
  annotationsByPage: {},
  currentColor: '#ffff00',
  strokeWidth: 3,
  setTool: (activeTool) => set({ activeTool }),
  addAnnotation: (ann) =>
    set((state) => ({
      annotationsByPage: {
        ...state.annotationsByPage,
        [ann.pageIndex]: [...(state.annotationsByPage[ann.pageIndex] ?? []), ann],
      },
    })),
  removeAnnotation: (pageIndex, id) =>
    set((state) => ({
      annotationsByPage: {
        ...state.annotationsByPage,
        [pageIndex]: (state.annotationsByPage[pageIndex] ?? []).filter((a) => a.id !== id),
      },
    })),
  setAnnotationsForPage: (pageIndex, anns) =>
    set((state) => ({
      annotationsByPage: { ...state.annotationsByPage, [pageIndex]: anns },
    })),
  clearAnnotations: () => set({ annotationsByPage: {} }),
  setColor: (currentColor) => set({ currentColor }),
  setStrokeWidth: (strokeWidth) => set({ strokeWidth }),
}));
