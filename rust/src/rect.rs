#[derive(Debug, Clone)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self {
            x,
            y,
            width: w,
            height: h,
        }
    }

    pub fn top(&self) -> f64 {
        self.y
    }

    pub fn bottom(&self) -> f64 {
        self.y + self.height
    }

    pub fn left(&self) -> f64 {
        self.x
    }

    pub fn right(&self) -> f64 {
        self.x + self.width
    }

    pub fn set_right(&mut self, x: f64) {
        self.x = x - self.width;
    }

    pub fn set_left(&mut self, x: f64) {
        self.x = x;
    }

    pub fn set_top(&mut self, y: f64) {
        self.y = y;
    }

    pub fn set_bottom(&mut self, y: f64) {
        self.y = y - self.height;
    }
}
