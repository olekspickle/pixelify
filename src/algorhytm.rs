//! # Algorhytms for pixelification
//!
//! Takes mutable image buffer and changes the pixels so that
//!

use image::{ImageBuffer, Pixel, Rgba};

//pub type ImageBuf<T> = ImageBuffer<T, Vec<u8>>;
pub type ImageBuf = ImageBuffer<Rgba<u8>, Vec<u8>>;

pub struct BoxBlur;

impl BoxBlur {
    //pub fn run<T: Pixel>(img: &mut ImageBuf<T>, scale: u32) -> &mut ImageBuf<T> {
    pub fn run(img: &mut ImageBuf, scale: u32) -> &mut ImageBuf {
        let w = img.width();
        let h = img.height();

        let uniform_grid = Self::uniform_grid(w, h, scale);
        for (x, y) in uniform_grid {
            Self::blend_rectangle(img, (x, y), scale);
        }
        img
    }

    // TODO: make generic for all images
    //fn blend_rectangle<T: Pixel>(img: &mut ImageBuf<T>, (x, y): (u32, u32), s: u32) {
    fn blend_rectangle(img: &mut ImageBuf, (x, y): (u32, u32), s: u32) {
        let w = img.width();
        let h = img.height();

        if x < s / 2 || y < s / 2 || x + 1 == w || y + 1 == h {
            return;
        }

        let (x1, y1) = (x.saturating_sub(s / 2), y.saturating_sub(s / 2));
        let (x2, y2) = ((x1 + s).min(w), (y1 + s).min(h));

        // Compute the average color of the region
        // TODO: figure out the better iterator version: cloning buffer for each pixel for big
        // images is madness
        //let (avg_r, avg_g, avg_b, avg_a, count) = (x1..x2)
        //    .flat_map(|xi| {
        //        let tmp = img.clone();
        //        (y1..y2).map(move |yi| {
        //            let px = tmp.get_pixel(xi, yi);
        //            *px
        //        })
        //    })
        //    .fold((0u64, 0u64, 0u64, 0u64, 0u64), |(r, g, b, a, n), pixel| {
        //        let [pr, pg, pb, pa] = pixel.0;
        //        (
        //            r + pr as u64,
        //            g + pg as u64,
        //            b + pb as u64,
        //            a + pa as u64,
        //            n + 1,
        //        )
        //    });
        let channels = img.get_pixel(0, 0);
        let mut r_sum = 0;
        let mut g_sum = 0;
        let mut b_sum = 0;
        let mut o_sum = 0;
        let mut n = 0;
        for x in x1..x2 {
            for y in y1..y2 {
                let pixel = img.get_pixel(x, y);
                let [r, g, b, o] = pixel.channels() else {
                    continue;
                };
                r_sum += *r as u32;
                g_sum += *g as u32;
                b_sum += *b as u32;
                o_sum += *o as u32;
                n += 1;
            }
        }

        if n == 0 {
            return;
        }

        let avg_color = image::Rgba([
            (r_sum / n) as u8,
            (g_sum / n) as u8,
            (b_sum / n) as u8,
            (o_sum / n) as u8,
        ]);

        // Apply the average color to the region
        (x1..x2)
            .flat_map(|xi| (y1..y2).map(move |yi| (xi, yi)))
            .for_each(|(xi, yi)| *img.get_pixel_mut(xi, yi) = avg_color);
    }

    fn uniform_grid(w: u32, h: u32, s: u32) -> Vec<(u32, u32)> {
        (s..w)
            .enumerate()
            .filter(|(i, _)| i % s as usize == 0)
            .flat_map(|x| {
                (s..h)
                    .enumerate()
                    .filter(|(i, _)| i % s as usize == 0)
                    .map(move |y| (x.1, y.1))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const S: u32 = 3;
    const W: u32 = 10;
    const H: u32 = 10;
    #[test]
    fn grid() {
        let grid = BoxBlur::uniform_grid(W, H, S);
        let expect: &[(u32, u32)] = &[
            (3, 3),
            (3, 6),
            (3, 9),
            (6, 3),
            (6, 6),
            (6, 9),
            (9, 3),
            (9, 6),
            (9, 9),
        ];
        assert_eq!(expect, grid);
    }

    #[test]
    fn run_works() {
        let mut img_buf = ImageBuf::new(W, H);
        let buf = img_buf.clone();
        let new_buf = BoxBlur::run(&mut img_buf, S);

        println!("buf {:?} vs {:?}", buf.clone(), new_buf.clone());
        let mut expect = ImageBuf::new(W, H);

        assert_eq!(&mut expect, new_buf);
    }
}
