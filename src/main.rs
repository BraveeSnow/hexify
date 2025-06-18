use clap::Parser;
use env_logger::Env;
use image::DynamicImage;
use log::info;

use crate::args::RuntimeArgs;

mod args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: RuntimeArgs = RuntimeArgs::parse();

    env_logger::builder()
        .parse_env(Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .init();

    info!(
        "Starting {} v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    // Open image file
    let image: DynamicImage = image::open(args.input)?;

    // Save processed image
    let processed_image = process_image(&image)?;
    processed_image.save(args.output)?;

    Ok(())
}

fn process_image(image: &DynamicImage) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    Ok(image.clone())
}
