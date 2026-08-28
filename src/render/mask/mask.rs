pub struct Mask {
    pub data: Vec<bool>,
    pub width: u32,
    pub height: u32,
}

impl Mask {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            data: vec![false; (width * height) as usize],
            width,
            height,
        }
    }

    pub fn set(&mut self, x: u32, y: u32, value: bool) {
        if x < self.width && y < self.height {
            self.data[(y * self.width + x) as usize] = value;
        }
    }

    pub fn get(&self, x: u32, y: u32) -> bool {
        if x < self.width && y < self.height {
            self.data[(y * self.width + x) as usize]
        } else {
            false
        }
    }

    pub fn fill_rect(&mut self, x: u32, y: u32, w: u32, h: u32, value: bool) {
        for row in y..(y + h).min(self.height) {
            for col in x..(x + w).min(self.width) {
                self.data[(row * self.width + col) as usize] = value;
            }
        }
    }

    pub fn invert(&mut self) {
        for val in &mut self.data {
            *val = !*val;
        }
    }
}
