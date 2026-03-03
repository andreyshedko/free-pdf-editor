import { describe, it, expect, beforeEach } from 'vitest';
import { useAnnotationStore } from './annotationStore';

describe('annotationStore', () => {
  beforeEach(() => {
    useAnnotationStore.setState({
      activeTool: null,
      annotationsByPage: {},
      currentColor: '#ffff00',
      strokeWidth: 3,
    });
  });

  it('setTool updates the active tool', () => {
    useAnnotationStore.getState().setTool('draw');
    expect(useAnnotationStore.getState().activeTool).toBe('draw');
  });

  it('setTool can clear the active tool to null', () => {
    useAnnotationStore.getState().setTool('highlight');
    useAnnotationStore.getState().setTool(null);
    expect(useAnnotationStore.getState().activeTool).toBeNull();
  });

  it('addAnnotation appends to the correct page', () => {
    const ann = { id: 'a1', pageIndex: 1, type: 'draw' as const, fabricJSON: '{}' };
    useAnnotationStore.getState().addAnnotation(ann);
    expect(useAnnotationStore.getState().annotationsByPage[1]).toEqual([ann]);
  });

  it('addAnnotation accumulates multiple annotations on the same page', () => {
    const ann1 = { id: 'a1', pageIndex: 0, type: 'draw' as const, fabricJSON: '{}' };
    const ann2 = { id: 'a2', pageIndex: 0, type: 'rect' as const, fabricJSON: '{}' };
    useAnnotationStore.getState().addAnnotation(ann1);
    useAnnotationStore.getState().addAnnotation(ann2);
    expect(useAnnotationStore.getState().annotationsByPage[0]).toHaveLength(2);
  });

  it('removeAnnotation removes the annotation with the given id', () => {
    const ann = { id: 'a1', pageIndex: 0, type: 'draw' as const, fabricJSON: '{}' };
    useAnnotationStore.getState().addAnnotation(ann);
    useAnnotationStore.getState().removeAnnotation(0, 'a1');
    expect(useAnnotationStore.getState().annotationsByPage[0]).toEqual([]);
  });

  it('setAnnotationsForPage replaces annotations for a page', () => {
    const ann1 = { id: 'a1', pageIndex: 0, type: 'draw' as const, fabricJSON: '{}' };
    const ann2 = { id: 'a2', pageIndex: 0, type: 'rect' as const, fabricJSON: '{}' };
    useAnnotationStore.getState().addAnnotation(ann1);
    useAnnotationStore.getState().setAnnotationsForPage(0, [ann2]);
    expect(useAnnotationStore.getState().annotationsByPage[0]).toEqual([ann2]);
  });

  it('clearAnnotations empties all annotations', () => {
    const ann = { id: 'a1', pageIndex: 0, type: 'draw' as const, fabricJSON: '{}' };
    useAnnotationStore.getState().addAnnotation(ann);
    useAnnotationStore.getState().clearAnnotations();
    expect(useAnnotationStore.getState().annotationsByPage).toEqual({});
  });

  it('setColor updates currentColor', () => {
    useAnnotationStore.getState().setColor('#ff0000');
    expect(useAnnotationStore.getState().currentColor).toBe('#ff0000');
  });

  it('setStrokeWidth updates strokeWidth', () => {
    useAnnotationStore.getState().setStrokeWidth(8);
    expect(useAnnotationStore.getState().strokeWidth).toBe(8);
  });
});
