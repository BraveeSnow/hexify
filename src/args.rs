use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct HexifyArgs {
    //
    // --- GENERAL SETTINGS ---
    /// The path to the source image to be used for processing.
    #[arg(short)]
    pub image: String,
    /// The path to save the processed image.
    #[arg(short, long)]
    pub output: String,
    /// The path to the JSON palette file.
    #[arg(short, long)]
    pub palette: String,

    //
    // --- ALGORITHM SETTINGS ---
    //
    /// The box radius of the average pixel RGB values
    #[arg(short, long, default_value_t = 1)]
    pub average: u32,

    //
    // --- MISCELLANEOUS SETTINGS ---
    //
    /// Displays logs at the INFO level.
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}
