const { contextBridge, ipcRenderer } = require('electron');

const api = {
  openPdf: async () => {
    return ipcRenderer.invoke('fs:openPdfDialog');
  },
  savePdf: async (data, name) => {
    return ipcRenderer.invoke('fs:savePdfDialog', data, name);
  }
};

contextBridge.exposeInMainWorld('electronAPI', api);
