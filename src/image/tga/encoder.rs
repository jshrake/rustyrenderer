use std::io;
use std::io::Write;
use byteorder::{WriteBytesExt, LittleEndian};

/// A type for encoding Truevision TGA images
pub struct Encoder<'a, W: 'a> {
    w: &'a mut W,
}

impl<'a, W: Write> Encoder<'a, W> {
    /// Create a new TGA `Encoder` backed by Writable `w`
    pub fn new(w: &mut W) -> Encoder<W> {
        Encoder { w: w }
    }

    /// Serialize a slice of `pixels` of dimension `width` x `height` as an uncompressed true-color
    /// TGA image
    pub fn encode(&mut self, pixels: &[u8], width: usize, height: usize) -> io::Result<()> {
        // -------
        // Header
        // -------
        let id_length: u8 = 0;
        let color_map_type: u8 = 0;
        // FIX(jshrake): uncompressed true-color image
        // Support other image_types
        let image_type: u8 = 2;
        // FIX(jshrake): No colormap support
        let color_map_origin_index: u16 = 0;
        let color_map_length: u16 = 0;
        let color_map_bits_per_pixel: u8 = 0;
        let image_x_origin: u16 = 0;
        let image_y_origin: u16 = 0;
        // NOTE(jshrake): Consider changing the signature of encode to take width and height as
        // u16?
        let image_width: u16 = width as u16;
        let image_height: u16 = height as u16;
        // FIX(jshrake): Assumes 4 bytes per pixel, 8 bits per byte
        let image_bits_per_pixel: u8 = 32;
        // Bits 3-0: Number of
        // Bit 4   : Must be 0
        // Bit 5   : Screen origin bit
        //              0 = Origin in lower left-hand corner
        //              1 = Origin in upper left-hand corner
        // Bits 7-6: Data storage interleaving flag
        // NOTE(jshrake): We want our origin in the upper left-hand corner
        let image_descriptor: u8 = 1 << 5;
        // NOTE(jshrake): Header description https://en.wikipedia.org/wiki/Truevision_TGA#Header
        try!(self.w.write_u8(id_length));
        try!(self.w.write_u8(color_map_type));
        try!(self.w.write_u8(image_type));
        try!(self.w.write_u16::<LittleEndian>(color_map_origin_index));
        try!(self.w.write_u16::<LittleEndian>(color_map_length));
        try!(self.w.write_u8(color_map_bits_per_pixel));
        try!(self.w.write_u16::<LittleEndian>(image_x_origin));
        try!(self.w.write_u16::<LittleEndian>(image_y_origin));
        try!(self.w.write_u16::<LittleEndian>(image_width));
        try!(self.w.write_u16::<LittleEndian>(image_height));
        try!(self.w.write_u8(image_bits_per_pixel));
        try!(self.w.write_u8(image_descriptor));
        // --------
        // Image data
        // --------
        // FIX(jshrake): Assumes BGRA format
        try!(self.w.write_all(pixels));
        // --------
        // Developer area
        // --------
        try!(self.w.write_all(&[0, 0, 0, 0]));
        // --------
        // Extension
        // --------
        try!(self.w.write_all(&[0, 0, 0, 0]));
        // --------
        // Footer
        // --------
        try!(self.w.write_all("TRUEVISION-XFILE.\0".as_bytes()));
        Ok(())
    }
}
