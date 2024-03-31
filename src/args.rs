use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct HexifyArgs {
    #[arg(short)]
    pub image: String,
    #[arg(short, long)]
    pub output: String,
    #[arg(short, long)]
    pub palette: String,
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}
