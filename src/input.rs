use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::Duration;
use crossterm::event::{self, Event, KeyCode};

pub struct Controls {
    pub paused: Arc<AtomicBool>,
    pub quit: Arc<AtomicBool>,
    pub reset: Arc<AtomicBool>,
}

impl Controls {
    pub fn new() -> Self {
        Self {
            paused: Arc::new(AtomicBool::new(false)),
            quit: Arc::new(AtomicBool::new(false)),
            reset: Arc::new(AtomicBool::new(false)),
        }
    }
}

/// Spawn input thread that listens for keys and sets flags:
/// - Space: toggle pause
/// - 'r': reset (set flag true for one loop)
/// - 'q' or Ctrl-C: quit
pub fn spawn_input_thread(ctrls: &Controls) {
    let paused = Arc::clone(&ctrls.paused);
    let quit = Arc::clone(&ctrls.quit);
    let reset = Arc::clone(&ctrls.reset);

    std::thread::spawn(move || {
        loop {
            // poll with small timeout so thread can be responsive
            if event::poll(Duration::from_millis(100)).unwrap_or(false) {
                if let Ok(Event::Key(key)) = event::read() {
                    match key.code {
                        KeyCode::Char(' ') => {
                            let cur = paused.load(Ordering::SeqCst);
                            paused.store(!cur, Ordering::SeqCst);
                        }
                        KeyCode::Char('r') | KeyCode::Char('R') => {
                            reset.store(true, Ordering::SeqCst);
                        }
                        KeyCode::Char('q') | KeyCode::Char('Q') => {
                            quit.store(true, Ordering::SeqCst);
                            break;
                        }
                        KeyCode::Esc => {
                            quit.store(true, Ordering::SeqCst);
                            break;
                        }
                        _ => {}
                    }
                }
            }
            if quit.load(Ordering::SeqCst) {
                break;
            }
        }
    });
}