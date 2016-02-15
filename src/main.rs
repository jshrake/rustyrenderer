#![feature(plugin)]
#![plugin(clippy)]
#![allow(dead_code)]
#![feature(test)]

extern crate byteorder;

mod image;
mod renderer;

use std::fs::File;
use std::path::Path;
use renderer::framebuffer::Framebuffer;
use renderer::Color;

fn main() {
    let (w, h) = (100, 100);
    let mut fb = Framebuffer::new(w, h);
    let color = Color::red();
    renderer::line(&mut fb, w - 1, 0, 0, h - 1, color);
    let mut fout = File::create(&Path::new("image.tga")).unwrap();
    fb.save(&mut fout, image::format::Format::Tga).unwrap();
}
