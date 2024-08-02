pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    background_color: u32,
    current_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
            background_color: 0x000000,
            current_color: 0xFF0000,
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
    }

    pub fn point(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = self.current_color;
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    pub fn render(&mut self, state: &mut Vec<Vec<bool>>) {
        let width = self.width;
        let height = self.height;
        let mut new_state = state.clone();

        for y in 0..height {
            for x in 0..width {
                let mut live_neighbors = 0;

                for dy in [-1, 0, 1].iter() {
                    for dx in [-1, 0, 1].iter() {
                        if *dx == 0 && *dy == 0 {
                            continue;
                        }

                        let nx = x as isize + dx;
                        let ny = y as isize + dy;

                        if nx >= 0 && ny >= 0 && nx < width as isize && ny < height as isize {
                            if state[ny as usize][nx as usize] {
                                live_neighbors += 1;
                            }
                        }
                    }
                }

                if state[y][x] {
                    if live_neighbors < 2 || live_neighbors > 3 {
                        new_state[y][x] = false;
                    }
                } else {
                    if live_neighbors == 3 {
                        new_state[y][x] = true;
                    }
                }

                if new_state[y][x] {
                    self.point(x, y);
                }
            }
        }

        *state = new_state;
    }
}
