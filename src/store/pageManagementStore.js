import { create } from 'zustand';

export const usePageManagementStore = create((set) => ({
  pageOrder: [],
  initPages: (count) => set({ pageOrder: Array.from({ length: count }, (_, i) => i) }),
  deletePage: (index) =>
    set((state) => ({ pageOrder: state.pageOrder.filter((_, i) => i !== index) })),
  reorderPage: (fromIndex, toIndex) =>
    set((state) => {
      const order = [...state.pageOrder];
      const [item] = order.splice(fromIndex, 1);
      order.splice(toIndex, 0, item);
      return { pageOrder: order };
    }),
  reset: () => set({ pageOrder: [] }),
}));
