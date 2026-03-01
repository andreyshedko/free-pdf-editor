export interface ElectronAPI {
  openPdf: () => Promise<{
    path: string;
    name: string;
    data: ArrayBuffer;
  } | null>;
}

declare global {
  interface Window {
    electronAPI?: ElectronAPI;
  }
}

