extern crate std;

use std::io;
use std::io::Read;
use std::io::BufRead;
use std::io::BufReader;


#[derive(Debug)]
pub struct FaceElement {
    pub vertex_index: u64,
    pub texture_index: Option<u64>,
    pub normal_index: Option<u64>,
}

#[derive(Debug)]
pub enum Token {
    Vertex(f32, f32, f32, f32),
    Face(Vec<FaceElement>),
}

/// A type for decoding Wavefront obj files
pub struct Decoder<'a, R: 'a> {
    r: &'a mut R,
}

impl<'a, R: Read> Decoder<'a, R> {
    /// Create a new obj `Decoder` backed by Readable `r`
    pub fn new(r: &mut R) -> Decoder<R> {
        Decoder { r: r }
    }

    pub fn decode(&mut self) -> Vec<Result<Token, Box<std::error::Error>>> {
        let mut reader = BufReader::new(&mut self.r);
        let mut buffer = String::new();
        let mut tokens = Vec::new();
        while reader.read_line(&mut buffer).unwrap() > 0 {
            if buffer.starts_with("v ") {
                tokens.push(parse_vertex(&buffer));
            } else if buffer.starts_with("f ") {
                tokens.push(parse_face(&buffer));
            }
            buffer.clear();
        }
        tokens
    }
}

fn parse_vertex(line: &str) -> Result<Token, Box<std::error::Error>> {
    let mut verts = line.split_whitespace();
    // Skip the leading "v"
    verts.next();
    let x: f32 = try!(verts.next().unwrap().parse());
    let y: f32 = try!(verts.next().unwrap().parse());
    let z: f32 = try!(verts.next().unwrap().parse());
    let w: f32 = try!(verts.next().unwrap_or("1.0").parse());
    Ok(Token::Vertex(x, y, z, w))
}

fn parse_face(line: &str) -> Result<Token, Box<std::error::Error>> {
    let mut groups = line.split_whitespace();
    // Skip the leading "f"
    groups.next();
    Ok(Token::Face(groups.map(|g| parse_face_element(g).unwrap()).collect()))
}

fn parse_face_element(group: &str) -> Result<FaceElement, Box<std::error::Error>> {
    let mut iters = group.split('/');
    let vi = iters.next().unwrap();
    // Some(iters.next().unwrap().parse().unwrap()),
    // Some(iters.next().unwrap().parse().unwrap()),
    Ok(FaceElement {
        vertex_index: vi.parse().unwrap(),
        texture_index: None,
        normal_index: None,
    })
}
