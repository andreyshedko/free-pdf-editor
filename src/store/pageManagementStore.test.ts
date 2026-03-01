import { describe, it, expect, beforeEach } from 'vitest';
import { usePageManagementStore } from './pageManagementStore';

describe('pageManagementStore', () => {
  beforeEach(() => {
    usePageManagementStore.getState().reset();
  });

  it('initPages sets sequential page order', () => {
    usePageManagementStore.getState().initPages(3);
    expect(usePageManagementStore.getState().pageOrder).toEqual([0, 1, 2]);
  });

  it('deletePage removes the page at the given index', () => {
    usePageManagementStore.getState().initPages(3);
    usePageManagementStore.getState().deletePage(1);
    expect(usePageManagementStore.getState().pageOrder).toEqual([0, 2]);
  });

  it('reorderPage moves a page from one position to another', () => {
    usePageManagementStore.getState().initPages(3);
    usePageManagementStore.getState().reorderPage(0, 2);
    expect(usePageManagementStore.getState().pageOrder).toEqual([1, 2, 0]);
  });

  it('reset clears the page order', () => {
    usePageManagementStore.getState().initPages(5);
    usePageManagementStore.getState().reset();
    expect(usePageManagementStore.getState().pageOrder).toEqual([]);
  });
});
