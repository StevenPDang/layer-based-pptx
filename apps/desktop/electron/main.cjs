const path = require('node:path');
const { app, BrowserWindow } = require('electron');
const {
  createWindowOptions,
  getDevServerUrl,
  isAllowedNavigation,
} = require('./window-options.cjs');

app.enableSandbox();

function createWindow() {
  const preloadPath = path.join(__dirname, 'preload.cjs');
  const window = new BrowserWindow(createWindowOptions(preloadPath));
  const devServerUrl = getDevServerUrl(process.argv);

  window.once('ready-to-show', () => window.show());
  window.webContents.setWindowOpenHandler(() => ({ action: 'deny' }));
  window.webContents.on('will-navigate', (event, targetUrl) => {
    if (!isAllowedNavigation(window.webContents.getURL(), targetUrl)) {
      event.preventDefault();
    }
  });

  if (devServerUrl) {
    void window.loadURL(devServerUrl);
  } else {
    void window.loadFile(path.join(__dirname, '..', 'dist', 'index.html'));
  }
}

app.whenReady().then(() => {
  createWindow();

  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      createWindow();
    }
  });
});

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});
