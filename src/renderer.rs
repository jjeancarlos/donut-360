use crate::state::{Buffers, BUF_SIZE, SCREEN_H, SCREEN_W};
use std::f32::consts::PI;
use std::io::{stdout, Write};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::{Duration, Instant};
use crossterm::{cursor, execute, terminal::{self, ClearType}};

/// Renderer holds rotation state and renders frames into the buffers.
/// It's designed to be executed in a dedicated thread.
pub struct Renderer {
    pub a: f32,
    pub b: f32,
    pub buffers: Buffers,
    pub paused: Arc<AtomicBool>,
    pub quit: Arc<AtomicBool>,
}

impl Renderer {
    pub fn new(buffers: Buffers, paused: Arc<AtomicBool>, quit: Arc<AtomicBool>) -> Self {
        Self {
            a: 0.0,
            b: 0.0,
            buffers,
            paused,
            quit,
        }
    }

    pub fn reset_rotation(&mut self) {
        self.a = 0.0;
        self.b = 0.0;
    }

    /// Run the render loop. This function blocks — intended to run in a thread.
    /// It will update FPS counter and print to terminal using crossterm.
    pub fn run(&mut self, target_frame_ms: u64, show_fps: bool, reset_flag: Arc<AtomicBool>) {
        // Setup terminal: enter raw mode, hide cursor
        let mut stdout = stdout();
        let _ = terminal::enable_raw_mode();
        let _ = execute!(stdout, terminal::Clear(ClearType::All), cursor::Hide);

        let mut last_fps_instant = Instant::now();
        let mut frames = 0u32;
        let mut fps_display = 0u32;

        while !self.quit.load(Ordering::SeqCst) {
            let start = Instant::now();

            // handle reset flag from input
            if reset_flag.load(Ordering::SeqCst) {
                self.reset_rotation();
                reset_flag.store(false, Ordering::SeqCst);
            }

            if !self.paused.load(Ordering::SeqCst) {
                // compute frame into buffers
                self.buffers.reset();
                self.compute_frame();
            }
            // regardless of paused, we still draw current buffers
            self.draw_frame(show_fps, fps_display);

            frames += 1;
            if last_fps_instant.elapsed().as_secs() >= 1 {
                fps_display = frames;
                frames = 0;
                last_fps_instant = Instant::now();
            }

            // sleep to cap FPS
            let elapsed_ms = start.elapsed().as_millis() as u64;
            if target_frame_ms > elapsed_ms {
                std::thread::sleep(Duration::from_millis(target_frame_ms - elapsed_ms));
            }
        }

        // Restore terminal
        let _ = execute!(std::io::stdout(), cursor::Show);
        let _ = terminal::disable_raw_mode();
    }

    fn compute_frame(&mut self) {
        // Implementation faithful to the C version:
        // two loops over angles, populate z and char buffers.
        let mut j = 0.0_f32;
        while j < 2.0 * PI {
            let mut i_val = 0.0_f32;
            while i_val < 2.0 * PI {
                let c = i_val.sin();
                let d = j.cos();
                let e = self.a.sin();
                let f = j.sin();
                let g = self.a.cos();
                let h = d + 2.0;
                let d_val = 1.0 / (c * h * e + f * g + 5.0);
                let l = i_val.cos();
                let m = self.b.cos();
                let n = self.b.sin();
                let t = c * h * g - f * e;

                let x = (40.0 + 30.0 * d_val * (l * h * m - t * n)) as isize;
                let y = (12.0 + 15.0 * d_val * (l * h * n + t * m)) as isize;
                let o = x + SCREEN_W as isize * y;

                if y >= 0 && y < SCREEN_H as isize && x >= 0 && x < SCREEN_W as isize {
                    let n_val = (8.0
                        * ((f * e - c * d * g) * m
                            - c * d * e
                            - f * g
                            - l * d * n)) as isize;

                    if o >= 0 && (o as usize) < BUF_SIZE {
                        let mut zlock = self.buffers.z.lock().unwrap();
                        if d_val > zlock[o as usize] {
                            zlock[o as usize] = d_val;
                            drop(zlock);

                            let chars = b".,-~:;=!*#$@";
                            let idx = if n_val > 0 { n_val as usize } else { 0 };
                            let ch = chars[idx.min(chars.len() - 1)] as char;

                            let mut clock = self.buffers.chars.lock().unwrap();
                            clock[o as usize] = ch;
                        }
                    }
                }

                i_val += 0.02;
            }
            j += 0.07;
        }

        // increment rotation *inside* frame printing (like original C) would be extremely fast;
        // instead we increment once per frame to keep smooth and stable speeds.
        // If you want exact original behavior, move increments to print loop.
        self.a += 0.04; // larger step so rotation is visible
        self.b += 0.02;
    }

    fn draw_frame(&self, show_fps: bool, fps: u32) {
        let mut stdout = std::io::stdout();

        // Move cursor home
        let _ = execute!(stdout, cursor::MoveTo(0, 0));

        // Optionally display status line
        if show_fps {
            // Show controls and FPS on top-left
            let paused_text = if self.paused.load(Ordering::SeqCst) { "[PAUSED]" } else { "        " };
            let _ = print!("{} FPS: {}  (space pause/resume, r reset, q quit)\n", paused_text, fps);
        }

        let chars = self.buffers.chars.lock().unwrap();
        // Print buffer: SCREEN_H lines of SCREEN_W characters
        for y in 0..SCREEN_H {
            for x in 0..SCREEN_W {
                let idx = y * SCREEN_W + x;
                let _ = print!("{}", chars[idx]);
            }
            let _ = print!("\r\n");
        }
        let _ = stdout.flush();
    }
}