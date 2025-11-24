mod renderer;
mod input;
mod state;

use renderer::Renderer;
use state::Buffers;
use input::{Controls, spawn_input_thread};

use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

fn main() {
    // Initialize shared structures
    let buffers = Buffers::new();
    let controls = Controls::new();

    // Spawn input thread
    spawn_input_thread(&controls);

    // Copy reset atomic into renderer (so renderer can consume resets)
    let reset_flag = Arc::clone(&controls.reset);

    // Create renderer and run it in main thread
    let mut renderer = Renderer::new(buffers, Arc::clone(&controls.paused), Arc::clone(&controls.quit));

    // Target ~30 FPS -> frame ms ~33
    let target_frame_ms = 33;

    // show_fps true to display FPS and controls
    renderer.run(target_frame_ms, true, reset_flag);

    // When quit is triggered, exit
    // ensure we set quit true to input thread if not already
    controls.quit.store(true, Ordering::SeqCst);

    // small delay to allow cleanup
    std::thread::sleep(Duration::from_millis(50));
}