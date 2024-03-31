use std::{fs, path::Path};

use log::error;

use super::types::{PaletteSettings, RGB};

// Defining a Color Scheme JSON File
//
// To create a custom color palette, create a new .json file. Inside the JSON
// object, create a key called "palette" with a list of hex colors.

pub fn get_color_palette(json_path: &Path) -> Vec<RGB> {
    let json_raw = fs::read_to_string(json_path);

    if let Err(err) = json_raw {
        error!("Unable to read from palette file: {}", err);
        return Vec::new();
    }

    let mut color_palette: Vec<RGB> = Vec::new();
    let json_serial = serde_json::from_str::<PaletteSettings>(json_raw.unwrap().as_str());

    if let Ok(json) = json_serial {
        for color in json.palette {
            let color = RGB::try_from(color);
            if let Ok(rgb) = color {
                color_palette.push(rgb);
            }
        }
    }

    color_palette
}
