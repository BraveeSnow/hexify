use image::RgbImage;

pub struct ProcessingSettings {
    pub average_radius: u32,
}

pub fn pixels_from_radius(img: &RgbImage, x: u32, y: u32, radius: u32) -> Vec<[u8; 3]> {
    let mut pixels: Vec<[u8; 3]> = Vec::new();

    for offset_x in x.checked_sub(radius + 1).unwrap_or(0)..(x + radius + 1).min(img.width()) {
        for offset_y in y.checked_sub(radius + 1).unwrap_or(0)..(y + radius + 1).min(img.height()) {
            pixels.push(img.get_pixel(offset_x, offset_y).0);
        }
    }

    pixels
}

pub fn average(pixels: &Vec<[u8; 3]>) -> [u8; 3] {
    let mut avg_pixel: [i32; 3] = [0, 0, 0];

    for pixel in pixels {
        avg_pixel[0] += pixel[0] as i32;
        avg_pixel[1] += pixel[1] as i32;
        avg_pixel[2] += pixel[2] as i32;
    }

    [
        (avg_pixel[0] / pixels.len() as i32) as u8,
        (avg_pixel[1] / pixels.len() as i32) as u8,
        (avg_pixel[2] / pixels.len() as i32) as u8,
    ]
}
