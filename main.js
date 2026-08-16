const { app, BrowserWindow, nativeImage } = require('electron');
const path = require('path');

// Disable hardware acceleration issues on some Windows machines
// app.disableHardwareAcceleration();

let mainWindow;

function createWindow() {
  // Load the app icon
  let appIcon;
  try {
    appIcon = nativeImage.createFromPath(path.join(__dirname, 'assets', 'icon.png'));
  } catch (e) {
    appIcon = undefined;
  }

  mainWindow = new BrowserWindow({
    width: 1400,
    height: 900,
    minWidth: 900,
    minHeight: 600,
    title: 'J.A.R.V.I.S — AI Assistant',
    icon: appIcon,
    backgroundColor: '#0e0e11',
    show: false, // Don't show until ready to prevent white flash
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true,
      preload: path.join(__dirname, 'preload.js'),
      // Allow media access (camera, mic) without prompts
      // Since this is a personal desktop app
      webSecurity: true,
      allowRunningInsecureContent: false,
    },
  });

  // Remove the default menu bar (clean look)
  mainWindow.setMenuBarVisibility(false);

  // Grant camera and mic permissions automatically (personal app)
  mainWindow.webContents.session.setPermissionRequestHandler(
    (webContents, permission, callback) => {
      const allowedPermissions = [
        'media',           // camera + mic
        'mediaKeySystem',
        'notifications',
        'fullscreen',
        'clipboard-read',
        'clipboard-sanitized-write',
      ];
      if (allowedPermissions.includes(permission)) {
        callback(true);
      } else {
        callback(false);
      }
    }
  );

  // Handle permission checks (some APIs check before requesting)
  mainWindow.webContents.session.setPermissionCheckHandler(
    (webContents, permission) => {
      const allowedChecks = ['media', 'notifications', 'clipboard-read'];
      return allowedChecks.includes(permission);
    }
  );

  // Load the JARVIS HTML file
  mainWindow.loadFile(path.join(__dirname, 'src', 'index.html'));

  // Show window when content is ready (no white flash)
  mainWindow.once('ready-to-show', () => {
    mainWindow.show();
  });

  mainWindow.on('closed', () => {
    mainWindow = null;
  });
}

// App lifecycle
app.whenReady().then(createWindow);

app.on('window-all-closed', () => {
  app.quit();
});

app.on('activate', () => {
  if (BrowserWindow.getAllWindows().length === 0) {
    createWindow();
  }
});
