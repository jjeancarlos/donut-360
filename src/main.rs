use std::{f32::consts::PI, thread, time::Duration};

fn main() {
    let mut a: f32 = 0.0;
    let mut b: f32 = 0.0;

    // Buffers
    let mut z_buffer = [0.0_f32; 1760];
    let mut char_buffer = [' '; 1760];

    print!("\x1b[2J"); // Clear terminal

    loop {
        // Reset dos buffers
        for i in 0..1760 {
            char_buffer[i] = ' ';
            z_buffer[i] = 0.0;
        }

        let mut j = 0.0;
        while j < 2.0 * PI {
            let mut i_val = 0.0;

            while i_val < 2.0 * PI {
                let c = i_val.sin();
                let d = j.cos();
                let e = a.sin();
                let f = j.sin();
                let g = a.cos();
                let h = d + 2.0;
                let d_val = 1.0 / (c * h * e + f * g + 5.0);
                let l = i_val.cos();
                let m = b.cos();
                let n = b.sin();
                let t = c * h * g - f * e;

                let x = (40.0 + 30.0 * d_val * (l * h * m - t * n)) as isize;
                let y = (12.0 + 15.0 * d_val * (l * h * n + t * m)) as isize;
                let o = x + 80 * y;

                if y >= 0 && y < 22 && x >= 0 && x < 80 {
                    let n_val = (8.0
                        * ((f * e - c * d * g) * m
                            - c * d * e
                            - f * g
                            - l * d * n)) as isize;

                    if o >= 0 && o < 1760 && d_val > z_buffer[o as usize] {
                        z_buffer[o as usize] = d_val;

                        const CHARS: &[u8] = b".,-~:;=!*#$@";
                        let idx = if n_val > 0 { n_val as usize } else { 0 };

                        char_buffer[o as usize] =
                            CHARS[idx.min(CHARS.len() - 1)] as char;
                    }
                }

                i_val += 0.02;
            }
            j += 0.07;
        }

        // Move cursor to home
        print!("\x1b[H");

        // Impressão + rotação a cada char (igual ao C)
        for k in 0..1760 {
            let c = char_buffer[k];

            if k % 80 == 79 {
                print!("\n");
            } else {
                print!("{}", c);
            }

            // Incrementos originais
            a += 0.00004;
            b += 0.00002;
        }

        thread::sleep(Duration::from_millis(30));
    }
}