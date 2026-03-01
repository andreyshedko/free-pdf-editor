import { contextBridge, ipcRenderer } from 'electron';

export interface OpenedPdf {
  path: string;
  name: string;
  data: ArrayBuffer;
}

const api = {
  openPdf: async (): Promise<OpenedPdf | null> => {
    return ipcRenderer.invoke('fs:openPdfDialog');
  }
};

declare global {
  interface Window {
    electronAPI: typeof api;
  }
}

contextBridge.exposeInMainWorld('electronAPI', api);

