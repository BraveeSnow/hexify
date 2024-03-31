mod algorithm;
mod args;
mod image;
mod palette;

use std::{io, path::PathBuf};

use ::image::io::Reader;
use args::HexifyArgs;
use clap::Parser;
use env_logger::Env;
use image::process_image;
use log::{error, info};
use palette::parser::get_color_palette;

use crate::algorithm::ProcessingSettings;

fn main() -> io::Result<()> {
    let args = HexifyArgs::parse();
    env_logger::builder()
        .parse_env(Env::default().default_filter_or(if args.verbose { "info" } else { "warn" }))
        .format_timestamp(None)
        .init();

    let palette_path = PathBuf::from(args.palette);
    let rgbs = get_color_palette(&palette_path);

    if rgbs.len() == 0 {
        error!("Palette is empty, terminating...");
        return Ok(());
    }

    let img_path = PathBuf::from(args.image);
    if !img_path.exists() {
        error!(
            "Source image {} does not exist, terminating...",
            img_path.display()
        );
        return Ok(());
    }

    let processed_image = process_image(
        &Reader::open(img_path).unwrap().decode().unwrap().into(),
        rgbs,
        ProcessingSettings {
            average_radius: args.average,
        },
    );

    info!("Saving image to {}", args.output);

    Ok(processed_image
        .save(args.output.to_owned())
        .or(Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            format!("Unable to write to path {}", args.output.to_owned()),
        )))?)
}
