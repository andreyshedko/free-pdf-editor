import { describe, it, expect, beforeEach } from 'vitest';
import { usePdfDocumentStore } from './pdfDocumentStore';

describe('pdfDocumentStore', () => {
  beforeEach(() => {
    usePdfDocumentStore.setState({
      fileData: null,
      currentFileName: null,
      pageCount: null,
      currentPageIndex: -1,
      zoom: 100,
      viewMode: 'fit-width',
    });
  });

  it('setDocument stores the file data and name', () => {
    const data = new ArrayBuffer(4);
    usePdfDocumentStore.getState().setDocument({ data, name: 'test.pdf' });
    const state = usePdfDocumentStore.getState();
    expect(state.fileData).toBe(data);
    expect(state.currentFileName).toBe('test.pdf');
  });

  it('setDocument resets pageCount to null and currentPageIndex to 0', () => {
    usePdfDocumentStore.getState().setDocument({ data: new ArrayBuffer(4), name: 'a.pdf' });
    const state = usePdfDocumentStore.getState();
    expect(state.pageCount).toBeNull();
    expect(state.currentPageIndex).toBe(0);
  });

  it('updateFileData replaces file data and resets pageCount', () => {
    usePdfDocumentStore.getState().setDocument({ data: new ArrayBuffer(4), name: 'a.pdf' });
    usePdfDocumentStore.getState().setPageCount(5);
    const newData = new ArrayBuffer(8);
    usePdfDocumentStore.getState().updateFileData(newData);
    expect(usePdfDocumentStore.getState().fileData).toBe(newData);
    expect(usePdfDocumentStore.getState().pageCount).toBeNull();
  });

  it('setPageCount sets the page count', () => {
    usePdfDocumentStore.getState().setPageCount(10);
    expect(usePdfDocumentStore.getState().pageCount).toBe(10);
  });

  it('setPageCount does not override an already-set page count', () => {
    usePdfDocumentStore.getState().setPageCount(10);
    usePdfDocumentStore.getState().setPageCount(20);
    expect(usePdfDocumentStore.getState().pageCount).toBe(10);
  });

  it('setCurrentPageIndex clamps to valid range', () => {
    usePdfDocumentStore.getState().setDocument({ data: new ArrayBuffer(4), name: 'a.pdf' });
    usePdfDocumentStore.getState().setPageCount(5);
    usePdfDocumentStore.getState().setCurrentPageIndex(3);
    expect(usePdfDocumentStore.getState().currentPageIndex).toBe(3);
  });

  it('setCurrentPageIndex clamps below 0 to 0', () => {
    usePdfDocumentStore.getState().setDocument({ data: new ArrayBuffer(4), name: 'a.pdf' });
    usePdfDocumentStore.getState().setPageCount(5);
    usePdfDocumentStore.getState().setCurrentPageIndex(-1);
    expect(usePdfDocumentStore.getState().currentPageIndex).toBe(0);
  });

  it('setCurrentPageIndex clamps above pageCount-1 to pageCount-1', () => {
    usePdfDocumentStore.getState().setDocument({ data: new ArrayBuffer(4), name: 'a.pdf' });
    usePdfDocumentStore.getState().setPageCount(5);
    usePdfDocumentStore.getState().setCurrentPageIndex(10);
    expect(usePdfDocumentStore.getState().currentPageIndex).toBe(4);
  });

  it('setZoom updates zoom and sets viewMode to custom', () => {
    usePdfDocumentStore.getState().setZoom(150);
    expect(usePdfDocumentStore.getState().zoom).toBe(150);
    expect(usePdfDocumentStore.getState().viewMode).toBe('custom');
  });

  it('setZoom clamps below 25 to 25', () => {
    usePdfDocumentStore.getState().setZoom(10);
    expect(usePdfDocumentStore.getState().zoom).toBe(25);
  });

  it('setZoom clamps above 200 to 200', () => {
    usePdfDocumentStore.getState().setZoom(300);
    expect(usePdfDocumentStore.getState().zoom).toBe(200);
  });

  it('setViewMode updates the view mode', () => {
    usePdfDocumentStore.getState().setViewMode('fit-page');
    expect(usePdfDocumentStore.getState().viewMode).toBe('fit-page');
  });
});
