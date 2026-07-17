use raylib::prelude::*;

pub struct FrameBuffer {
    image: Image,
    color: Color,
}

impl FrameBuffer {
    pub fn new(width: i32, height: i32, background: Color) -> Self {
        FrameBuffer {
            image: Image::gen_image_color(width, height, background),
            color: Color::WHITE,
        }
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn point(&mut self, x: i32, y: i32) {
        if x >= 0 && x < self.image.width() && y >= 0 && y < self.image.height() {
            self.image.draw_pixel(x, y, self.color);
        }
    }

    pub fn render(&self, filename: &str) {
        self.image.export_image(filename);
    }
}
