extern crate std;
use renderer::framebuffer::Framebuffer;
use renderer::Color;

pub fn line(buffer: &mut Framebuffer,
            mut x0: usize,
            mut y0: usize,
            mut x1: usize,
            mut y1: usize,
            color: Color) {
    let run = (x1 as i32 - x0 as i32).abs();
    let rise = (y1 as i32 - y0 as i32).abs();
    let mut transposed = false;
    if run < rise {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
        transposed = true;
    }
    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }
    x1 = std::cmp::min(x1, buffer.width());
    y1 = std::cmp::min(y1, buffer.height());
    let dx = x1 as i32 - x0 as i32;
    let dy = y1 as i32 - y0 as i32;
    let derror = dy.abs() * 2;
    let mut error = 0;
    let mut y = y0 as i32;
    let inc = if y1 > y0 {
        1i32
    } else {
        -1i32
    };
    for x in x0..x1 + 1 {
        if transposed {
            buffer.set_pixel(y as usize, x, color);
        } else {
            buffer.set_pixel(x, y as usize, color);
        }
        error += derror;
        if error > dx {
            y += inc;
            error -= dx * 2;
        }
    }
}

#[cfg(test)]
mod test {
    extern crate test;
    use renderer::framebuffer::Framebuffer;
    use renderer::Color;
    use renderer::line::line;

    #[test]
    fn vertical() {
        let (w, h) = (10, 10);
        let mut fb = Framebuffer::new(w, h);
        let color = Color::red();
        line(&mut fb, w / 2, 0, w / 2, h - 1, color);
        for px in 0..h {
            assert!(fb.get_pixel(w / 2, px) == color);
        }
    }

    #[test]
    fn horizontal() {
        let (w, h) = (10, 10);
        let mut fb = Framebuffer::new(w, h);
        let color = Color::red();
        line(&mut fb, 0, h / 2, w - 1, h / 2, color);
        for px in 0..w {
            assert!(fb.get_pixel(px, h / 2) == color);
        }
    }

    #[test]
    fn top_left_to_bottom_right() {
        let (w, h) = (10, 10);
        let mut fb = Framebuffer::new(w, h);
        let color = Color::red();
        line(&mut fb, 0, 0, w - 1, h - 1, color);
        for px in 0..w {
            assert!(fb.get_pixel(px, px) == color);
        }
    }

    #[test]
    fn bottom_right_to_top_left() {
        let (w, h) = (10, 10);
        let mut fb = Framebuffer::new(w, h);
        let color = Color::red();
        line(&mut fb, w - 1, h - 1, 0, 0, color);
        for px in 0..w {
            assert!(fb.get_pixel(px, px) == color);
        }
    }

    #[test]
    fn top_right_to_bottom_left() {
        let (w, h) = (10, 10);
        let mut fb = Framebuffer::new(w, h);
        let color = Color::red();
        line(&mut fb, w - 1, 0, 0, h - 1, color);
        for px in 0..w {
            assert!(fb.get_pixel((w - 1) - px, px) == color);
        }
    }

    #[test]
    fn bottom_left_to_top_right() {
        let (w, h) = (10, 10);
        let mut fb = Framebuffer::new(w, h);
        let color = Color::red();
        line(&mut fb, 0, h - 1, w - 1, 0, color);
        for px in 0..w {
            assert!(fb.get_pixel((w - 1) - px, px) == color);
        }
    }

    #[bench]
    fn bench_line(b: &mut test::Bencher) {
        let (w, h) = (100, 100);
        let mut fb = Framebuffer::new(w, h);
        let color = Color::red();
        b.iter(|| line(&mut fb, 0, h - 1, w - 1, 0, color));
    }

}
