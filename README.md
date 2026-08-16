# J.A.R.V.I.S — Persistent Windows Desktop Assistant (Tauri v2)

Welcome to the persistent Windows Desktop transformation for **J.A.R.V.I.S**. This setup wraps your existing single-file JARVIS application with a Rust-powered Tauri shell, creating a floating desktop orb widget, system tray integration, offline voice wake-word listening, expressive human voice speech synthesis, and cross-session task memory persistence.

---

## 📁 Complete Updated File Tree

```
jarvis ultimate/
├── package.json                  # Added Tauri scripts & devDependencies
├── README.md                     # Setup, usage & architecture guide
├── assets/
│   ├── icon.png                  # Main application icon
│   └── logo.png                  # Glowing sci-fi orb logo asset
├── src/
│   ├── index.html                # Original JARVIS UI (CONFIG_VERSION=5 + Tauri Bridge)
│   └── orb.html                  # Floating Orb desktop widget UI
└── src-tauri/
    ├── Cargo.toml                # Rust dependencies (tauri, winapi, cpal, screenshots, open)
    ├── tauri.conf.json           # Tauri v2 multi-window, system tray, hotkeys & autostart config
    ├── build.rs                  # Standard Tauri build script
    └── src/
        ├── main.rs               # Desktop application entry point
        ├── lib.rs                # App setup, tray events, global shortcut Ctrl+Shift+J
        ├── commands.rs           # OS commands (open app, run command, FS, volume, screenshot)
        ├── voice.rs              # Continuous offline wake-word listener ("Jarvis")
        └── storage.rs            # Persistent JSON storage (%APPDATA%/jarvis/orb_position.json & task_memory.json)
```

---

## ⚙️ Prerequisites

1. **Node.js** (v18 or higher)
2. **Rust Toolchain**: Install via [rustup.rs](https://rustup.rs/) (`stable-x86_64-pc-windows-msvc`).
3. **C++ Build Tools**: Visual Studio Build Tools with "Desktop development with C++" workload installed.

---

## 🚀 How to Run & Build

### 1. Install Node Dependencies
```bash
npm install
```

### 2. Launch Development Mode
Run the development environment. This compiles the Rust backend and opens the floating desktop orb overlay along with the main JARVIS UI:
```bash
npm run tauri:dev
```

### 3. Build Standalone Executable / Installer (.exe / .msi)
To generate the production Windows binary:
```bash
npm run tauri:build
```
The output `.exe` / `.msi` installers will be located in:
`src-tauri/target/release/bundle/`

---

## 🔮 How the Floating Orb & Key Features Work

### 1. Always-On-Top Floating Orb Overlay
- **Continuous Desktop Presence**: The orb appears on your screen as a 64x64 transparent circular widget. It stays **always-on-top**, floating over any open browser tab, game, or software application.
- **Draggable Everywhere**: Click and drag the orb anywhere on your monitor. Its exact `(X, Y)` position is saved persistently in `%APPDATA%/jarvis/orb_position.json` across system restarts.
- **Expand / Collapse**: Click or tap the orb at any time to smoothly expand or hide the full JARVIS chat interface anchored near the orb.

### 2. System Tray & Auto-Start
- Closing the main chat window hides it to the Windows system tray (near the clock) rather than quitting.
- Right-click the system tray icon to access:
  - **Show/Hide J.A.R.V.I.S**
  - **Settings**
  - **Quit**
- Auto-start on Windows startup is managed via `tauri-plugin-autostart`.

### 3. Global Hotkey (`Ctrl+Shift+J`)
- Press `Ctrl+Shift+J` from anywhere on Windows (even when another app is focused) to immediately toggle the JARVIS interface.

### 4. Continuous Offline Voice Wake-Word ("Jarvis")
- The Rust audio thread monitors your microphone continuously.
- When you speak **"Jarvis"**, the orb glows intensely in cyan/green, expands the chat UI, greets you with *"Yes Boss? I am listening."*, and automatically triggers speech recognition.

### 5. Persistent Task Memory
- Whenever you work with JARVIS, your queries, active tasks, and context are saved into `%APPDATA%/jarvis/task_memory.json`.
- If you ask: **"Yes Jarvis, what were we working on?"**, JARVIS automatically recalls the saved task state from app data and tells you exactly what you were working on!

### 6. Human-like Expressive Voice
- Speech synthesis uses natural voice selection with dynamic inflection tuning (adjusting pitch, rate, and stress based on question/statement structure) for a human-like computer voice.

### 7. OS-Level Control Commands (`window.jarvisDesktop`)
- Open Installed App: `window.jarvisDesktop.openApp("notepad")`
- Run Shell Command: `window.jarvisDesktop.runCommand("dir")` *(Prompts confirmation for destructive commands like `del` / `rm`)*
- Read / Write Files: `window.jarvisDesktop.readUserFile(path)`, `window.jarvisDesktop.writeUserFile(path, content)`
- System Volume: `window.jarvisDesktop.setVolume(0.8)`
- Desktop Screenshot: `window.jarvisDesktop.takeScreenshot()`
