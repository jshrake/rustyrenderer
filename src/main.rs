#![feature(plugin)]
#![plugin(clippy)]

extern crate byteorder;

mod image;
mod framebuffer;

use std::fs::File;
use std::path::Path;

fn main() {
    let mut image = framebuffer::Framebuffer::new(3, 3);
    image.set_pixel(0, 0, 255, 0, 0, 255);
    image.set_pixel(1, 0, 0, 255, 0, 255);
    image.set_pixel(2, 0, 0, 0, 255, 255);
    image.set_pixel(0, 2, 0, 0, 255, 255);
    image.set_pixel(1, 2, 0, 255, 0, 255);
    image.set_pixel(2, 2, 255, 0, 0, 255);
    let mut fout = File::create(&Path::new("image.tga")).unwrap();
    image.save(&mut fout, image::format::Format::Tga).unwrap();
}
