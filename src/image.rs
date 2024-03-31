use image::RgbImage;
use log::info;

use crate::{
    algorithm::{average, pixels_from_radius, ProcessingSettings},
    palette::types::RGB,
};

pub fn process_image(img: &RgbImage, palette: Vec<RGB>, settings: ProcessingSettings) -> RgbImage {
    let mut processed = RgbImage::new(img.width(), img.height());

    info!("Got image with dimensions {}x{}", img.width(), img.height());

    for i in 0..img.width() {
        for j in 0..img.height() {
            let mut index = 0;
            let mut distance = i32::MAX;

            for color_index in 0..palette.len() {
                // its fine to keep this here if average_radius = 0, it will
                // just retrieve the pixel at the given coordinate
                let avg_rgb = average(&pixels_from_radius(img, i, j, settings.average_radius));

                let color_distance = (palette[color_index].r as i32 - avg_rgb[0] as i32).pow(2)
                    + (palette[color_index].g as i32 - avg_rgb[1] as i32).pow(2)
                    + (palette[color_index].b as i32 - avg_rgb[2] as i32).pow(2);

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
