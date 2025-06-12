use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct RuntimeArgs {
    /// The path to the image to process
    #[arg(short, long)]
    pub input: String,

    /// The name of the newly created image
    #[arg(short, long)]
    pub output: String,
}
