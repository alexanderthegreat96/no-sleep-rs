use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use enigo::{Coordinate, Direction, Enigo, Key, Keyboard, Mouse, Settings};
use rand::Rng;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread;
use std::time::Duration;

// moves in a smooth S-curve pattern
// some voodoo shit if you ask me but yeah
// i still love rust lol
fn smooth_move(enigo: &mut Enigo, steps: u32, delay_ms: u64, forward: bool, stop: &AtomicBool) {
    let start_x = 100i32;
    let end_x = 1820i32;
    let start_y = 400i32;
    let mut rng = rand::rng();

    for step in 0..=steps {
        if stop.load(Ordering::Relaxed) {
            break;
        }

        let t = if forward {
            step as f32 / steps as f32
        } else {
            1.0 - (step as f32 / steps as f32)
        };

        // just horizontal screen travel
        let x = start_x + ((end_x - start_x) as f32 * t) as i32;
        //s shaped synwave
        let s_curve = (std::f32::consts::PI * t).sin() * 150.0;
        let y = start_y + s_curve as i32;

        // jitters every 5 steps
        let jitter_x = if step % 5 == 0 {
            rng.random_range(-1..=1)
        } else {
            0
        };
        let jitter_y = if step % 5 == 0 {
            rng.random_range(-1..=1)
        } else {
            0
        };

        let _ = enigo.move_mouse(x + jitter_x, y + jitter_y, Coordinate::Abs);
        thread::sleep(Duration::from_millis(delay_ms));
    }
}

pub fn esc_pressed() -> bool {
    if event::poll(Duration::from_millis(100)).unwrap_or(false) {
        if let Ok(Event::Key(key_event)) = event::read() {
            return key_event.code == KeyCode::Esc;
        }
    }
    false
}

pub fn run() {
    if let Err(e) = enable_raw_mode() {
        tracing::error!(
            "Warning: Failed to enable raw mode: {}. ESC detection may not work.",
            e
        );
        return;
    }

    // shared stop flag: threads run while this is false, and exit when true
    let stop = Arc::new(AtomicBool::new(false));

    // mouse thread
    let stop_mouse = Arc::clone(&stop);
    let mouse_handle = thread::spawn(move || {
        let mut enigo = match Enigo::new(&Settings::default()) {
            Ok(e) => e,
            Err(e) => {
                tracing::error!("Mouse thread: failed to initialize Enigo: {}", e);
                return;
            }
        };

        while !stop_mouse.load(Ordering::Relaxed) {
            // forward
            smooth_move(&mut enigo, 200, 2, true, &stop_mouse);
            if stop_mouse.load(Ordering::Relaxed) {
                break;
            }
            // reverse (come back following the reverse path)
            smooth_move(&mut enigo, 200, 2, false, &stop_mouse);
            if stop_mouse.load(Ordering::Relaxed) {
                break;
            }
            // small pause between cycles
            thread::sleep(Duration::from_millis(200));
        }
    });

    // key press thread: presses random modifier (Ctrl/Alt/Shift) every 10-60s
    let stop_keys = Arc::clone(&stop);
    let key_handle = thread::spawn(move || {
        let mut enigo = match Enigo::new(&Settings::default()) {
            Ok(e) => e,
            Err(e) => {
                tracing::error!("Key thread: failed to initialize Enigo: {}", e);
                return;
            }
        };

        let mut rng = rand::rng();
        while !stop_keys.load(Ordering::Relaxed) {
            // choose random interval between 10 and 60 seconds
            let interval_secs = rng.random_range(10..=60);
            // wait in small slices so we can exit quickly if stop requested
            let slices = (interval_secs * 5) as u64; // 200ms slices
            for _ in 0..slices {
                if stop_keys.load(Ordering::Relaxed) {
                    return;
                }
                thread::sleep(Duration::from_millis(200));
            }

            if stop_keys.load(Ordering::Relaxed) {
                break;
            }

            let choice = rng.random_range(0..3);
            let _ = match choice {
                0 => match enigo.key(Key::Control, Direction::Click) {
                    Ok(_) => tracing::info!("Pressed Ctrl key"),
                    Err(e) => {
                        tracing::error!("Failed to press Ctrl key: {}", e);
                    }
                },
                1 => match enigo.key(Key::Alt, Direction::Click) {
                    Ok(_) => tracing::info!("Pressed Alt key"),
                    Err(e) => {
                        tracing::error!("Failed to press Alt key: {}", e);
                    }
                },
                _ => {
                    match enigo.key(Key::Shift, Direction::Click) {
                        Ok(_) => tracing::info!("Pressed Shift key"),
                        Err(e) => {
                            tracing::error!("Failed to press Shift key: {}", e);
                        }
                    };
                }
            };
        }
    });
    tracing::info!("Keep-alive started: mouse + key threads. Press ESC to stop.\r\n");

    // main loop: monitor for ESC key to stop
    while !stop.load(Ordering::Relaxed) {
        if esc_pressed() {
            tracing::info!("ESC pressed, stopping threads...\r\n");
            stop.store(true, Ordering::Relaxed);
            if let Err(e) = disable_raw_mode() {
                tracing::warn!("Warning: Failed to disable raw mode: {}", e);
            }
            std::process::exit(0);
        }
        thread::sleep(Duration::from_millis(100));
    }

    // signal threads to stop and wait for them to finish
    stop.store(true, Ordering::Relaxed);
    let _ = mouse_handle.join();
    let _ = key_handle.join();

    if let Err(e) = disable_raw_mode() {
        tracing::warn!("Warning: Failed to disable raw mode: {}", e);
    }

    tracing::info!("Keep-alive stopped.");
}
