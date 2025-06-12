use clap::Parser;
use env_logger::Env;
use log::info;

use crate::args::RuntimeArgs;

mod args;

fn main() {
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
}
