"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
const electron_1 = require("electron");
const path = __importStar(require("path"));
const fs = __importStar(require("fs"));
const isDev = process.env.NODE_ENV === 'development';
let mainWindow = null;
function createMainWindow() {
    mainWindow = new electron_1.BrowserWindow({
        width: 1280,
        height: 800,
        minWidth: 1024,
        minHeight: 640,
        webPreferences: {
            preload: path.join(__dirname, 'preload.js'),
            contextIsolation: true,
            nodeIntegration: false,
            sandbox: false
        },
        title: 'Free PDF Editor'
    });
    if (isDev) {
        mainWindow.loadURL('http://localhost:5173');
        mainWindow.webContents.openDevTools({ mode: 'detach' });
    }
    else {
        const indexPath = path.join(__dirname, '..', '..', 'dist', 'index.html');
        mainWindow.loadFile(indexPath);
    }
    mainWindow.on('closed', () => {
        mainWindow = null;
    });
}
electron_1.app.on('ready', () => {
    createMainWindow();
});
electron_1.app.on('window-all-closed', () => {
    if (process.platform !== 'darwin') {
        electron_1.app.quit();
    }
});
electron_1.app.on('activate', () => {
    if (electron_1.BrowserWindow.getAllWindows().length === 0) {
        createMainWindow();
    }
});
electron_1.ipcMain.handle('fs:openPdfDialog', async () => {
    const { canceled, filePaths } = await electron_1.dialog.showOpenDialog({
        filters: [{ name: 'PDF Files', extensions: ['pdf'] }],
        properties: ['openFile']
    });
    if (canceled || filePaths.length === 0) {
        return null;
    }
    const filePath = filePaths[0];
    const data = await fs.promises.readFile(filePath);
    return {
        path: filePath,
        name: path.basename(filePath),
        data: data.buffer.slice(data.byteOffset, data.byteOffset + data.byteLength)
    };
});
electron_1.ipcMain.handle('fs:savePdfDialog', async (_event, data, defaultName) => {
    const { canceled, filePath } = await electron_1.dialog.showSaveDialog({
        defaultPath: defaultName,
        filters: [{ name: 'PDF Files', extensions: ['pdf'] }]
    });
    if (canceled || !filePath)
        return false;
    await fs.promises.writeFile(filePath, Buffer.from(data));
    return true;
});
