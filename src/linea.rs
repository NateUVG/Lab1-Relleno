use crate::framebuffer::FrameBuffer;

pub struct Linea {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
}

impl Linea {
    pub fn new(x0: i32, y0: i32, x1: i32, y1: i32) -> Self {
        Linea { x0, y0, x1, y1 }
    }

    pub fn bresenham(&self, fb: &mut FrameBuffer) {
        let mut x = self.x0;
        let mut y = self.y0;

        let dx = (self.x1 - self.x0).abs();
        let dy = -(self.y1 - self.y0).abs();
        let sx = if self.x0 < self.x1 { 1 } else { -1 };
        let sy = if self.y0 < self.y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            fb.point(x, y);
            if x == self.x1 && y == self.y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }
}
