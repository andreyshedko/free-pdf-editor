import { contextBridge, ipcRenderer } from 'electron';

export interface OpenedPdf {
  path: string;
  name: string;
  data: ArrayBuffer;
}

const api = {
  openPdf: async (): Promise<OpenedPdf | null> => {
    return ipcRenderer.invoke('fs:openPdfDialog');
  },
  savePdf: async (data: ArrayBuffer, name: string): Promise<boolean> => {
    return ipcRenderer.invoke('fs:savePdfDialog', data, name);
  }
};

declare global {
  interface Window {
    electronAPI: typeof api;
  }
}

contextBridge.exposeInMainWorld('electronAPI', api);

