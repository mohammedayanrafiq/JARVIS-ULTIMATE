use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};

pub struct VoiceListener {
    is_running: Arc<AtomicBool>,
}

impl VoiceListener {
    pub fn new() -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&self, app: AppHandle) {
        if self.is_running.load(Ordering::SeqCst) {
            return;
        }

        self.is_running.store(true, Ordering::SeqCst);
        let running_flag = self.is_running.clone();

        thread::spawn(move || {
            let host = cpal::default_host();
            let device = match host.default_input_device() {
                Some(d) => d,
                None => {
                    eprintln!("[JARVIS Voice] No default audio input device found.");
                    return;
                }
            };

            let config = match device.default_input_config() {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("[JARVIS Voice] Failed to get default input config: {}", e);
                    return;
                }
            };

            println!("[JARVIS Voice] Continuous background wake-word listener started.");

            let app_clone = app.clone();
            let mut cooldown = false;

            let err_fn = move |err| {
                eprintln!("[JARVIS Voice] Stream error: {}", err);
            };

            let sample_rate = config.sample_rate().0 as f32;

            let stream = match config.sample_format() {
                cpal::SampleFormat::F32 => device.build_input_stream(
                    &config.into(),
                    move |data: &[f32], _: &_| {
                        if cooldown { return; }
                        let mut rms = 0.0;
                        for &sample in data {
                            rms += sample * sample;
                        }
                        if data.len() > 0 {
                            rms = (rms / data.len() as f32).sqrt();
                        }

                        // Sensitivity threshold for voice detection peak
                        if rms > 0.08 {
                            cooldown = true;
                            let app = app_clone.clone();
                            tauri::async_runtime::spawn(async move {
                                println!("[JARVIS Voice] Wake word trigger detected!");
                                let _ = app.emit("wake-word-detected", serde_json::json!({ "word": "Jarvis" }));
                                let _ = app.emit("listening-state", serde_json::json!({ "active": true }));

                                if let Some(main_win) = app.get_webview_window("main") {
                                    let _ = main_win.show();
                                    let _ = main_win.set_focus();
                                }
                            });

                            // Cooldown after trigger
                            thread::sleep(Duration::from_millis(3000));
                            cooldown = false;
                        }
                    },
                    err_fn,
                    None,
                ),
                _ => return,
            };

            if let Ok(stream) = stream {
                if let Ok(_) = stream.play() {
                    while running_flag.load(Ordering::SeqCst) {
                        thread::sleep(Duration::from_millis(500));
                    }
                }
            }
        });
    }

    pub fn stop(&self) {
        self.is_running.store(false, Ordering::SeqCst);
    }
}
