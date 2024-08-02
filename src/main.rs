mod framebuffer;

use minifb::{Window, WindowOptions};
use std::time::Duration;
use framebuffer::Framebuffer;

fn main() {
    let window_width = 800;
    let window_height = 800;

    let framebuffer_width = 100;
    let framebuffer_height = 100;

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Conway's Game of Life",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    fn set_glider(state: &mut Vec<Vec<bool>>, x: isize, y: isize) {
        let coords = [(x, y), (x + 1, y + 1), (x + 1, y + 2), (x, y + 2), (x - 1, y + 2)];
        for &(ix, iy) in &coords {
            if ix >= 0 && iy >= 0 && ix < state[0].len() as isize && iy < state.len() as isize {
                state[iy as usize][ix as usize] = true;
            }
        }
    }

    fn set_lwss(state: &mut Vec<Vec<bool>>, x: isize, y: isize) {
        let coords = [(x, y), (x + 3, y), (x + 4, y + 1), (x, y + 2), (x + 4, y + 2), (x + 1, y + 3), (x + 2, y + 3), (x + 3, y + 3)];
        for &(ix, iy) in &coords {
            if ix >= 0 && iy >= 0 && ix < state[0].len() as isize && iy < state.len() as isize {
                state[iy as usize][ix as usize] = true;
            }
        }
    }

    fn set_block(state: &mut Vec<Vec<bool>>, x: isize, y: isize) {
        let coords = [(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)];
        for &(ix, iy) in &coords {
            if ix >= 0 && iy >= 0 && ix < state[0].len() as isize && iy < state.len() as isize {
                state[iy as usize][ix as usize] = true;
            }
        }
    }

    fn set_blinker(state: &mut Vec<Vec<bool>>, x: isize, y: isize) {
        let coords = [(x, y), (x, y + 1), (x, y + 2)];
        for &(ix, iy) in &coords {
            if ix >= 0 && iy >= 0 && ix < state[0].len() as isize && iy < state.len() as isize {
                state[iy as usize][ix as usize] = true;
            }
        }
    }

    fn draw_car(state: &mut Vec<Vec<bool>>, x: isize, y: isize) {
        let coords = [
            (x, y), (x + 1, y), (x + 2, y), (x + 3, y), (x + 4, y),
            (x + 1, y + 1), (x + 2, y + 1), (x + 3, y + 1),
            (x + 2, y + 2),
            (x, y + 3), (x + 1, y + 3), (x + 2, y + 3), (x + 3, y + 3), (x + 4, y + 3),
        ];
        for &(ix, iy) in &coords {
            if ix >= 0 && iy >= 0 && ix < state[0].len() as isize && iy < state.len() as isize {
                state[iy as usize][ix as usize] = true;
            }
        }
    }

    let mut state = vec![vec![false; framebuffer_width]; framebuffer_height];
    let mut car_x: isize = 0;
    let car_y: isize = framebuffer_height as isize / 2;
    let mut car_active = true;

    set_glider(&mut state, 10, 10);
    set_lwss(&mut state, 20, 20);
    set_block(&mut state, 30, 30);
    set_blinker(&mut state, 40, 40);

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        framebuffer.clear();

        if car_active {
            if car_x < framebuffer_width as isize {
                draw_car(&mut state, car_x, car_y);
                car_x += 1;
            } else {
                car_active = false;
            }
        } else {
            framebuffer.render(&mut state);
        }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(Duration::from_millis(50));
    }
}
