#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Color::from_rgba(r, g, b, 255)
    }

    pub fn red() -> Color {
        Color::from_rgb(255, 0, 0)
    }

    pub fn green() -> Color {
        Color::from_rgb(0, 255, 0)
    }

    pub fn blue() -> Color {
        Color::from_rgb(0, 0, 255)
    }

    pub fn white() -> Color {
        Color::from_rgb(255, 255, 255)
    }

    pub fn black() -> Color {
        Color::from_rgb(0, 0, 0)
    }

    pub fn gray(v: u8) -> Color {
        Color::from_rgb(v, v, v)
    }

    pub fn rgba(&self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }
}
