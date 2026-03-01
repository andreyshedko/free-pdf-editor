import { create } from 'zustand';

export type AnnotationTool = 'select' | 'draw' | 'highlight' | 'text' | 'rect' | 'ellipse' | 'signature' | null;

export interface Annotation {
  id: string;
  pageIndex: number;
  type: AnnotationTool;
  fabricJSON: string;
}

export interface AnnotationState {
  activeTool: AnnotationTool;
  annotationsByPage: Record<number, Annotation[]>;
  currentColor: string;
  strokeWidth: number;
  setTool: (tool: AnnotationTool) => void;
  addAnnotation: (ann: Annotation) => void;
  removeAnnotation: (pageIndex: number, id: string) => void;
  setAnnotationsForPage: (pageIndex: number, anns: Annotation[]) => void;
  clearAnnotations: () => void;
  setColor: (color: string) => void;
  setStrokeWidth: (width: number) => void;
}

export const useAnnotationStore = create<AnnotationState>((set) => ({
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
