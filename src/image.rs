use image::RgbImage;

use crate::palette::types::RGB;

pub fn process_image(img: RgbImage, palette: Vec<RGB>) -> RgbImage {
    let mut processed = RgbImage::new(img.width(), img.height());

    for i in 0..img.width() {
        for j in 0..img.height() {
            let mut index = 0;
            let mut distance = i32::MAX;
            let channels = img.get_pixel(i, j);

            for color_index in 0..palette.len() {
                let color_distance = (palette[color_index].r as i32 - channels.0[0] as i32).pow(2)
                    + (palette[color_index].g as i32 - channels.0[1] as i32).pow(2)
                    + (palette[color_index].b as i32 - channels.0[2] as i32).pow(2);

                if color_distance < distance {
                    distance = color_distance;
                    index = color_index;
                }
            }

            processed.get_pixel_mut(i, j).0 =
                [palette[index].r, palette[index].g, palette[index].b];
        }
    }

    processed
}
