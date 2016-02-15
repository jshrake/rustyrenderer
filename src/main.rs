#![feature(plugin)]
#![plugin(clippy)]

extern crate byteorder;

mod image;
mod framebuffer;

use std::fs::File;
use std::path::Path;

fn line(buffer: &mut framebuffer::Framebuffer,
        x0: usize,
        y0: usize,
        x1: usize,
        y1: usize,
        r: u8,
        g: u8,
        b: u8,
        a: u8) {
    let step_count = 1000;
    for step in 0..step_count {
        let t = step as f32 / step_count as f32;
        let x = (x0 as f32 * (1.0 - t) + t * x1 as f32) as usize;
        let y = (y0 as f32 * (1.0 - t) + t * y1 as f32) as usize;
        buffer.set_pixel(x, y, r, g, b, a);
    }
}

fn main() {
    let mut fb = framebuffer::Framebuffer::new(100, 100);
    line(&mut fb, 0, 0, 100, 100, 255, 0, 0, 255);
    let mut fout = File::create(&Path::new("image.tga")).unwrap();
    fb.save(&mut fout, image::format::Format::Tga).unwrap();
}
