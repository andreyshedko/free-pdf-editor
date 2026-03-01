"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const electron_1 = require("electron");
const api = {
    openPdf: async () => {
        return electron_1.ipcRenderer.invoke('fs:openPdfDialog');
    },
    savePdf: async (data, name) => {
        return electron_1.ipcRenderer.invoke('fs:savePdfDialog', data, name);
    }
};
electron_1.contextBridge.exposeInMainWorld('electronAPI', api);
