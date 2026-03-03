export interface ElectronAPI {
  openPdf: () => Promise<{
    path: string;
    name: string;
    data: ArrayBuffer;
  } | null>;
  savePdf?: (data: ArrayBuffer, name: string) => Promise<boolean>;
}

declare global {
  interface Window {
    electronAPI?: ElectronAPI;
  }
}

