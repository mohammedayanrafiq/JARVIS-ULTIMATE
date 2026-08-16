# J.A.R.V.I.S Ultimate — Always-On Desktop AI Companion

> **The floating orb that never leaves your screen.**  
> Talk to it anytime. It listens. It answers with real expression. It remembers.

This is not another chatbot in a browser tab.  
This is **J.A.R.V.I.S Ultimate** — a true Windows desktop presence powered by Tauri v2 + Rust.

A glowing sci-fi orb lives permanently on your desktop.  
It stays always-on-top. You can drag it anywhere.  
Say **“Jarvis”** and it wakes up, glows, expands, and talks back with natural, expressive voice.

Built by **Mohammed Ayan Rafiq** (13) — AI Builder & Teenpreneur.

---

## Why this version feels different

- **Always visible floating Orb** — never buried under windows
- **Wake-word listening** that actually works offline in the background
- **Expressive human-like voice** with dynamic pitch, rate & emotion
- **Persistent memory** across sessions (it remembers what you were working on)
- **System-level control** (open apps, volume, screenshots, files, commands)
- **Global hotkey** `Ctrl + Shift + J` from anywhere
- **System tray + auto-start** so it feels like a real OS citizen

This is the closest thing to having Tony Stark’s J.A.R.V.I.S living on your Windows PC right now.

---

## Feature Highlights

### The Floating Orb
- Transparent, always-on-top, 64–80 px glowing orb
- Drag it anywhere — position is saved forever
- Click → expands the full JARVIS interface next to it
- Listening state = intense cyan/green pulse + faster ring animation
- Looks premium and sci-fi (logo.png / icon.png assets included)

### Voice That Actually Feels Alive
- Continuous offline wake-word detection (“Jarvis”)
- When triggered: orb glows, main window appears, greets you
- Expressive TTS with natural inflection (not robotic)
- You can talk to it anytime without clicking anything

### Memory & Context
- Saves active task + conversation history in `%APPDATA%/jarvis/`
- Ask “What were we working on?” → it tells you exactly

### Real Desktop Power
- Open any installed app
- Run shell commands (with safety confirmation for destructive ones)
- Read/write files
- Control system volume
- Take desktop screenshots
- Native Windows notifications

### Developer Experience
- Clean Tauri v2 multi-window architecture
- Rust backend for performance + security
- Easy to extend with more commands

---

## Project Structure

```
JARVIS-ULTIMATE/
├── package.json
├── README.md
├── assets/
│   ├── icon.png          # App & tray icon
│   └── logo.png          # Glowing orb logo
├── src/
│   ├── index.html        # Full JARVIS chat UI + Tauri bridge
│   └── orb.html          # Floating always-on-top orb widget
└── src-tauri/
    ├── Cargo.toml
    ├── tauri.conf.json
    ├── build.rs
    └── src/
        ├── main.rs
        ├── lib.rs                # Tray, global shortcut, orb restore, voice start
        ├── commands.rs           # All OS-level commands
        ├── voice.rs              # Continuous wake-word listener
        └── storage.rs            # Persistent orb position + task memory
```

---

## Prerequisites

1. **Node.js** v18+
2. **Rust** (stable-x86_64-pc-windows-msvc) → https://rustup.rs
3. **Visual Studio Build Tools** with “Desktop development with C++”

---

## Quick Start

```bash
# 1. Install dependencies
npm install

# 2. Run in development (orb + main window appear)
npm run tauri:dev

# 3. Build real Windows installer
npm run tauri:build
```

Output will be in `src-tauri/target/release/bundle/`

---

## How to Use Once Running

- The **orb** appears immediately and stays on top
- Drag it wherever you want
- Say **“Jarvis”** or press `Ctrl + Shift + J` or click the orb
- Talk naturally — it listens and replies with expression
- Close the main window → it goes to system tray (doesn’t quit)
- Right-click tray icon for Show/Hide / Settings / Quit

---

## Screenshots & Assets

All the premium screenshots (chat interface, voice mode, lock screen, calculators, food scanner, vitals, etc.) are kept in the root of the repo so the GitHub page looks rich and professional.

The new floating orb uses the high-quality `assets/logo.png` and `assets/icon.png`.

---

## Built By

**Mohammed Ayan Rafiq**  
13-year-old AI Builder & Teenpreneur  
Breaking things, reverse-engineering, and shipping real AI tools.

Contact: ayanimranrafiq@gmail.com  
Phone: +91 9742515282 (IND)

---

## License

MIT — feel free to fork, improve, and make your own personal J.A.R.V.I.S even better.

---

**This is the version that finally feels alive.**  
An orb that never leaves. A voice that answers with presence.  
Welcome to the future of personal AI on the desktop.
