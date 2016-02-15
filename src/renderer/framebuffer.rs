use std::io;
use std::io::Write;

use image::format::Format;
use image::tga;

use renderer::color::Color;

/// A rendering destination
pub struct Framebuffer {
    width: usize,
    height: usize,
    color_buffer: Vec<u8>,
}


impl Framebuffer {
    /// Create a new `Framebuffer` of size `width` x `height`
    pub fn new(width: usize, height: usize) -> Framebuffer {
        Framebuffer {
            width: width,
            height: height,
            color_buffer: vec![255; width*height*4],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    /// Set the `[x, y]` pixel of the color buffer to `Color`
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let start = 4 * (y * self.width + x);
        let (r, g, b, a) = color.rgba();
        self.color_buffer[start] = b;
        self.color_buffer[start + 1] = g;
        self.color_buffer[start + 2] = r;
        self.color_buffer[start + 3] = a;
    }

    /// Get the `Color` of the `[x, y]` pixel
    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        let start = 4 * (y * self.width + x);
        Color::from_rgba(self.color_buffer[start + 2],
                         self.color_buffer[start + 1],
                         self.color_buffer[start],
                         self.color_buffer[start + 3])
    }

    /// Serialize the `Framebuffer` according to the specified `format`
    pub fn save<W: Write>(&self, w: &mut W, format: Format) -> io::Result<()> {
        match format {
            Format::Tga => {
                let mut enc = tga::encoder::Encoder::new(w);
                try!(enc.encode(&self.color_buffer, self.width, self.height));
                Ok(())
            }
        }
    }
}
