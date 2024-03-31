mod args;
mod image;
mod palette;

use std::{io, path::PathBuf};

use ::image::io::Reader;
use args::HexifyArgs;
use clap::Parser;
use env_logger::Env;
use image::process_image;
use log::info;
use palette::parser::get_color_palette;

fn main() -> io::Result<()> {
    let args = HexifyArgs::parse();
    env_logger::builder()
        .parse_env(Env::default().default_filter_or(if args.verbose { "info" } else { "warn" }))
        .format_timestamp(None)
        .init();

    let palette_path = PathBuf::from(args.palette);
    let rgbs = get_color_palette(&palette_path);
    let processed_image = process_image(
        Reader::open(PathBuf::from(args.image))
            .unwrap()
            .decode()
            .unwrap()
            .into(),
        rgbs,
    );

    info!("Saving image to {}", args.output);

    Ok(processed_image
        .save(args.output.to_owned())
        .or(Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            format!("Unable to write to path {}", args.output.to_owned()),
        )))?)
}
