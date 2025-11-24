use std::sync::{Arc, Mutex};

pub const SCREEN_W: usize = 80;
pub const SCREEN_H: usize = 22;
pub const BUF_SIZE: usize = SCREEN_W * SCREEN_H; // 1760

#[derive(Clone)]
pub struct Buffers {
    // Arc+Mutex para acessar entre threads
    pub z: Arc<Mutex<Vec<f32>>>,
    pub chars: Arc<Mutex<Vec<char>>>,
}

impl Buffers {
    pub fn new() -> Self {
        Self {
            z: Arc::new(Mutex::new(vec![0.0_f32; BUF_SIZE])),
            chars: Arc::new(Mutex::new(vec![' '; BUF_SIZE])),
        }
    }

    pub fn reset(&self) {
        if let Ok(mut v) = self.z.lock() {
            for x in v.iter_mut() { *x = 0.0; }
        }
        if let Ok(mut c) = self.chars.lock() {
            for ch in c.iter_mut() { *ch = ' '; }
        }
    }
}