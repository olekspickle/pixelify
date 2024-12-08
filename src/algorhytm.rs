//! # Algorhytms for pixelification
//!
//! Takes mutable image buffer and changes the pixels so that
//!

use image::{
    error::{ImageFormatHint, UnsupportedError},
    ImageError, Pixel, RgbaImage,
};

pub struct BoxBlur;

impl BoxBlur {
    pub fn run(buf: &[u8], scale: u32) -> Result<RgbaImage, ImageError> {
        let format = image::guess_format(buf)?;
        let error = |s: &str| {
            ImageError::Unsupported(UnsupportedError::from_format_and_kind(
                ImageFormatHint::Exact(format),
                image::error::UnsupportedErrorKind::GenericFeature(s.to_owned()),
            ))
        };
        //println!("{buf:?}\n, format:{format:?}");

        let mut img: RgbaImage = image::load_from_memory_with_format(buf, format)
            .expect("Failed to load image")
            .into();

        let w = img.width();
        let h = img.height();
        if scale >= w || scale >= h {
            return Err(error("\nScale is bigger that the image"));
        }
        if scale <= 1 {
            return Err(error("\nScale must be bigger than 1 pixel"));
        }
        if scale > w / 2 || scale > h / 2 || w % scale != 0 || h % scale != 0 {
            return Err(error(
                "\nScale too big for this image, result will be ugly.
                Best results will be in range: 3 - width/3 or 3 - height/3.
                Additionally, scale should be so that width/height are divisible by scale evenly.",
            ));
        }

        let uniform_grid = Self::uniform_grid(w, h, scale);
        for (x, y) in uniform_grid {
            Self::blend_rectangle(&mut img, (x, y), scale);
        }

        Ok(img)
    }

    fn blend_rectangle(img: &mut RgbaImage, (x, y): (u32, u32), s: u32) {
        let w = img.width();
        let h = img.height();

        if x + 1 == w || y + 1 == h {
            return;
        }

        let (x1, y1) = (x.saturating_sub(s / 2), y.saturating_sub(s / 2));
        let (x2, y2) = ((x1 + s).min(w), (y1 + s).min(h));

        // Compute the blend color of the region
        // TODO: figure out the better iterator version: cloning the whole buffer for each pixel
        // for big images is memory madness
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

        let blend_color = image::Rgba([
            (r_sum / n) as u8,
            (g_sum / n) as u8,
            (b_sum / n) as u8,
            (o_sum / n) as u8,
        ]);

        // Apply the average color to the region
        (x1..x2)
            .flat_map(|xi| (y1..y2).map(move |yi| (xi, yi)))
            .for_each(|(xi, yi)| *img.get_pixel_mut(xi, yi) = blend_color);
    }

    /// Basically get scale bounded grids
    fn uniform_grid(w: u32, h: u32, s: u32) -> Vec<(u32, u32)> {
        (0..=w)
            .enumerate()
            .filter(|(i, _)| i % s as usize == 0)
            .flat_map(|x| {
                (0..=h)
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
    use image::EncodableLayout;

    const S: u32 = 3;
    const W: u8 = 10;
    const H: u8 = 10;
    const TEN_BY_TEN: &[u8; 499] = &[
        137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 10, 0, 0, 0, 10, 8,
        6, 0, 0, 0, 141, 50, 207, 189, 0, 0, 0, 9, 112, 72, 89, 115, 0, 0, 0, 0, 0, 0, 0, 1, 0,
        132, 121, 23, 115, 0, 0, 1, 165, 73, 68, 65, 84, 120, 156, 1, 154, 1, 101, 254, 0, 224,
        229, 232, 6, 160, 171, 178, 5, 227, 237, 243, 1, 228, 238, 244, 0, 225, 237, 243, 0, 227,
        238, 244, 0, 234, 242, 248, 0, 226, 233, 238, 13, 237, 242, 248, 0, 237, 243, 248, 0, 0,
        172, 184, 183, 61, 133, 143, 145, 91, 218, 227, 232, 19, 248, 250, 252, 0, 249, 250, 252,
        0, 244, 246, 247, 4, 190, 199, 197, 59, 177, 192, 193, 59, 229, 235, 240, 9, 220, 226, 230,
        18, 0, 88, 114, 89, 183, 76, 95, 75, 218, 156, 182, 186, 68, 207, 229, 242, 0, 210, 227,
        238, 1, 178, 196, 201, 38, 91, 117, 99, 187, 116, 143, 129, 132, 137, 159, 152, 107, 123,
        144, 134, 101, 0, 53, 85, 52, 255, 48, 79, 41, 255, 95, 132, 102, 175, 151, 177, 177, 62,
        136, 160, 158, 93, 89, 116, 95, 186, 55, 86, 53, 250, 60, 96, 62, 233, 53, 83, 54, 235, 51,
        82, 50, 217, 0, 41, 77, 42, 255, 49, 86, 42, 255, 63, 107, 60, 249, 78, 113, 82, 216, 76,
        108, 81, 210, 44, 79, 44, 255, 40, 71, 41, 255, 38, 74, 43, 255, 32, 66, 38, 255, 32, 66,
        36, 255, 0, 29, 59, 31, 255, 42, 76, 37, 255, 48, 92, 47, 255, 44, 82, 47, 255, 42, 77, 48,
        255, 33, 67, 39, 255, 32, 60, 35, 255, 29, 61, 36, 255, 29, 63, 37, 255, 25, 58, 31, 255,
        0, 24, 51, 25, 255, 37, 73, 35, 255, 44, 87, 45, 255, 32, 65, 36, 255, 25, 55, 32, 255, 23,
        50, 29, 255, 24, 53, 30, 255, 23, 53, 31, 255, 28, 63, 36, 255, 19, 51, 27, 255, 0, 23, 51,
        23, 255, 33, 72, 32, 255, 48, 92, 44, 255, 33, 66, 35, 255, 17, 46, 27, 255, 34, 61, 36,
        255, 55, 81, 49, 255, 57, 81, 45, 255, 83, 103, 54, 255, 66, 82, 39, 255, 0, 26, 62, 26,
        255, 30, 69, 31, 255, 41, 84, 39, 255, 29, 58, 31, 255, 49, 74, 42, 255, 116, 141, 74, 255,
        159, 184, 95, 255, 161, 187, 95, 255, 163, 186, 90, 255, 159, 175, 86, 255, 0, 22, 53, 24,
        255, 25, 52, 25, 255, 74, 100, 50, 255, 111, 136, 68, 255, 152, 182, 85, 255, 162, 192, 83,
        255, 156, 185, 74, 255, 155, 184, 71, 255, 146, 176, 69, 255, 149, 175, 70, 255, 9, 196,
        199, 203, 147, 129, 101, 95, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
    ];
    const FIVE_BY_FIVE: &[u8; 196] = &[
        137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 5, 0, 0, 0, 5, 8, 6,
        0, 0, 0, 141, 111, 38, 229, 0, 0, 0, 9, 112, 72, 89, 115, 0, 0, 0, 0, 0, 0, 0, 1, 0, 132,
        121, 23, 115, 0, 0, 0, 116, 73, 68, 65, 84, 120, 156, 1, 105, 0, 150, 255, 0, 172, 182,
        184, 46, 22, 3, 232, 239, 2, 237, 244, 250, 0, 208, 218, 222, 29, 227, 233, 239, 6, 0, 68,
        96, 68, 224, 153, 178, 173, 83, 155, 176, 172, 84, 84, 113, 90, 196, 96, 122, 103, 157, 0,
        38, 73, 36, 255, 58, 98, 59, 242, 46, 79, 51, 246, 28, 61, 33, 255, 21, 56, 29, 255, 0, 31,
        65, 30, 255, 32, 72, 36, 255, 24, 52, 30, 255, 47, 75, 44, 255, 53, 78, 41, 255, 0, 28, 61,
        28, 255, 64, 95, 47, 25, 5, 116, 144, 69, 255, 154, 181, 81, 255, 152, 174, 77, 255, 183,
        69, 49, 191, 241, 233, 80, 17, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
    ];

    #[test]
    fn grid() {
        let grid = BoxBlur::uniform_grid(W as u32, H as u32, S);
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
        let buf = std::fs::read("test.png").unwrap();
        let rgba = BoxBlur::run(TEN_BY_TEN, 2).unwrap();

        println!("buf {FIVE_BY_FIVE:?} vs {:?}", rgba.clone());

        assert_eq!(FIVE_BY_FIVE, rgba.as_bytes());
    }
}
