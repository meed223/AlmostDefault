use image::{GenericImageView, ImageBuffer, Pixel, Rgba, RgbaImage};

use crate::UpscalingParameters;

pub(crate) fn pixel_doubling_upscale(img: &ImageBuffer<Rgba<u8>, Vec<u8>>, scale: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut upscaled_img: image::ImageBuffer<Rgba<u8>, Vec<u8>> = RgbaImage::new(img.width() * scale, img.height() * scale);

    let mut y_offset = 0;
    let mut x_offset;
    let mut current_pixel: &Rgba<u8>;

    for y in 0..img.height() {
        x_offset = 0;
        for x in 0..img.width() {
            current_pixel = img.get_pixel(x, y);
            for sy in 0..scale {
                for sx in 0..scale {
                    upscaled_img.put_pixel(x + x_offset + sx, y + y_offset + sy, *current_pixel);
                }
            }
            x_offset += scale - 1;
        }
        y_offset += scale - 1;
    }
    
    return upscaled_img;
}

pub(crate) fn median_upscale(img: &ImageBuffer<Rgba<u8>, Vec<u8>>, upscaling_parameters: &UpscalingParameters) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, &'static str> {
    let upscaled_img = pixel_doubling_upscale(img, upscaling_parameters.scale as u32);
    let mut filtered_upscaled_img = upscaled_img.clone();
    let wb = (upscaling_parameters.median -1) / 2;

    let mut colours: Vec<&Rgba<u8>>;

    for x in wb..(upscaled_img.width() as i32 - wb) {
        for y in wb..(upscaled_img.height() as i32 - wb) {
            colours = Vec::new();
            // Inner loop to get 3x3 pixels around target pixel
            for i in -wb..=wb {
                for j in -wb..=wb {
                    // TODO: Need to add the bitwise add?
                    colours.push(upscaled_img.get_pixel((x + i) as u32, (y + j) as u32));
                }
            }
            let mean_colour = get_mean_colour(colours);
            filtered_upscaled_img.put_pixel(x as u32, y as u32, mean_colour);
        }
    }

    return Ok(filtered_upscaled_img);
 }

 pub(crate) fn get_mean_colour(colours: Vec<&Rgba<u8>>) -> Rgba<u8> {
    let mut r = Vec::new();
    let mut g = Vec::new();
    let mut b = Vec::new();
    let mut a = Vec::new();

    let mut channels;
    for c in colours {
        channels = c.channels();
        r.push(channels[0]);
        g.push(channels[1]);
        b.push(channels[2]);
        a.push(channels[3]);
    }

    r.sort_unstable();
    g.sort_unstable();
    b.sort_unstable();
    a.sort_unstable();

    let mean_channels = [r[r.len() / 2], g[g.len() / 2], b[b.len() / 2], a[a.len() / 2]];

    return Rgba::from(mean_channels);
}

pub(crate) fn circular_filter(source_img: &ImageBuffer<Rgba<u8>, Vec<u8>>, mut upscaled_img: ImageBuffer<Rgba<u8>, Vec<u8>>, upscaling_parameters: &UpscalingParameters) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, &'static str> {
    for y in 0..upscaled_img.height() {
        for x in 0..upscaled_img.width() {
            // TODO: Try changing to Option<T> ?
            let result = match compare_ssse(upscaling_parameters.scale, y as i32 % upscaling_parameters.scale, x as i32 % upscaling_parameters.scale) {
                Some(b) => b,
                None => return Err("Error: Unable to perform circular comparison.")
            };
            if result {
                upscaled_img.put_pixel(x, y, *source_img.get_pixel(x / upscaling_parameters.scale as u32,y / upscaling_parameters.scale as u32));
            }
        }
    }
    return Ok(upscaled_img);
}

// TODO: Refactor Rename
pub fn compare_ssse(scale: i32, y_percent_scale: i32, x_percent_scale: i32) -> Option<bool>{
    if scale == 2 {
        return Some(true);
    } else if scale == 4 && y_percent_scale <= 4 && x_percent_scale <= 4 {
        return Some(compare_4x(y_percent_scale, x_percent_scale));
    } else if scale == 8 && y_percent_scale <= 8 && x_percent_scale <= 8 {
        return Some(compare_8x(y_percent_scale, x_percent_scale));
    } else if scale == 16 && y_percent_scale <= 16 && x_percent_scale <= 16 {
        return Some(compare_16x(y_percent_scale, x_percent_scale));
    } else {
        return None;
    }
}

fn compare_4x(y_percent_scale: i32, x_percent_scale: i32) -> bool {
    let array: [[bool; 4]; 4] = [
        [false, true, true, false],
        [true, true, true, true],
        [true, true, true, true], 
        [false, true, true, false]
        ];

    return array[y_percent_scale as usize][x_percent_scale as usize]
}

fn compare_8x(y_percent_scale: i32, x_percent_scale: i32) -> bool {
    let array: [[bool; 8]; 8] = [
        [false, false, true, true, true, true, false, false],
        [false, true, true, true, true, true, true, false],
        [true, true, true, true, true, true, true, true],
        [true, true, true, true, true, true, true, true],
        [true, true, true, true, true, true, true, true],
        [true, true, true, true, true, true, true, true],
        [false, true, true, true, true, true, true, false],
        [false, false, true, true, true, true, false, false]
        ];

    return array[y_percent_scale as usize][x_percent_scale as usize]
}

fn compare_16x(y_percent_scale: i32, x_percent_scale: i32) -> bool {
    let array: [[bool; 16]; 16] = [
        [ false, false, false, false, false, true, true, true, true, true, true, false, false, false, false, false ],
        [ false, false, false, true, true, true, true, true, true, true, true, true, true, false, false, false ],
        [ false, false, true, true, true, true, true, true, true, true, true, true, true, true, false, false ],
        [ false, true, true, true, true, true, true, true, true, true, true, true, true, true, true, false ],
        [ false, true, true, true, true, true, true, true, true, true, true, true, true, true, true, false ],
        [ true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true ],
        [ true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true ],
        [ true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true ],
        [ true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true ],
        [ true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true ],
        [ true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true ],
        [ false, true, true, true, true, true, true, true, true, true, true, true, true, true, true, false ],
        [ false, true, true, true, true, true, true, true, true, true, true, true, true, true, true, false ],
        [ false, false, true, true, true, true, true, true, true, true, true, true, true, true, false, false ],
        [ false, false, false, true, true, true, true, true, true, true, true, true, true, false, false, false ],
        [ false, false, false, false, false, true, true, true, true, true, true, false, false, false, false, false ] 
        ];

    return array[y_percent_scale as usize][x_percent_scale as usize]
}