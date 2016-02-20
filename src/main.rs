#![feature(plugin)]
#![plugin(clippy)]
#![allow(dead_code)]
#![feature(test)]

extern crate byteorder;

mod image;
mod renderer;
mod obj;

use std::fs::File;
use std::path::Path;
use renderer::framebuffer::Framebuffer;
use renderer::Color;
use obj::decoder;

fn main() {

    let mut objin = File::open(&Path::new("obj/african_head.obj")).unwrap();
    let mut obj = decoder::Decoder::new(&mut objin);
    let mut verts = Vec::<f32>::new();
    let mut faces = Vec::new();
    for token in obj.decode() {
        match token {
            Ok(decoder::Token::Vertex(x, y, _, _)) => verts.extend_from_slice(&[x, y]),
            Ok(decoder::Token::Face(els)) => faces.push(els),
            _ => {}
        }
    }
    let (w, h) = (800, 800);
    let mut fb = Framebuffer::new(w, h);
    let mut fout = File::create(&Path::new("images/african_head.tga")).unwrap();
    for face in faces {
        for j in 0..3 {
            let v0 = 2 * (face[j].vertex_index as usize - 1);
            let v1 = 2 * (face[(j + 1) % 3].vertex_index as usize - 1);
            let x0 = (verts[v0] + 1.0) * (fb.width() as f32 / 2.0);
            let y0 = (verts[v0 + 1] + 1.0) * (fb.height() as f32 / 2.0);
            let x1 = (verts[v1] + 1.0) * (fb.width() as f32 / 2.0);
            let y1 = (verts[v1 + 1] + 1.0) * (fb.height() as f32 / 2.0);
            renderer::line(&mut fb,
                           x0.round() as usize,
                           y0.round() as usize,
                           x1.round() as usize,
                           y1.round() as usize,
                           Color::black());
        }
    }
    fb.save(&mut fout, image::format::Format::Tga).unwrap();
}
