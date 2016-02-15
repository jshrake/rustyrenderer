use std::io;
use std::io::Write;

use image::format::Format;
use image::tga;

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

    /// Set the `[x, y]` pixel of the color buffer to `[r, g, b, a]`
    pub fn set_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8, a: u8) {
        let start = 4 * (y * self.width + x);
        self.color_buffer[start] = b;
        self.color_buffer[start + 1] = g;
        self.color_buffer[start + 2] = r;
        self.color_buffer[start + 3] = a;
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
