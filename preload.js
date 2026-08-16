// Preload script — runs in an isolated context before the web page loads.
// Phase 1: Minimal — no bridge yet. Just ensures contextIsolation works.
// Phase 2 will add the command bridge (shell, filesystem, apps, screenshots) here.

const { contextBridge } = require('electron');

// Expose a minimal API so the frontend knows it's running inside Electron
contextBridge.exposeInMainWorld('jarvisDesktop', {
  platform: process.platform,
  isElectron: true,
  version: '1.0.0',
});
