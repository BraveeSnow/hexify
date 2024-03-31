use log::warn;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PaletteSettings {
    pub palette: Vec<String>,
}

#[derive(Debug)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct ColorError;

impl TryFrom<String> for RGB {
    type Error = ColorError;

    fn try_from(hex: String) -> Result<RGB, Self::Error> {
        let mut trimmed = hex.trim().to_string();

        if trimmed.starts_with("#") {
            trimmed.remove(0);
        }

        if trimmed.len() != 6 {
            warn!("Unable to parse hex color {}, skipping...", trimmed);
            return Err(ColorError);
        }

        Ok(RGB {
            r: u8::from_str_radix(&trimmed[0..2], 16).or(Err(ColorError))?,
            g: u8::from_str_radix(&trimmed[2..4], 16).or(Err(ColorError))?,
            b: u8::from_str_radix(&trimmed[4..6], 16).or(Err(ColorError))?,
        })
    }
}
