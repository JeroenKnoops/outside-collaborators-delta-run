use anyhow::Result;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

mod commands;
use commands::*;

mod file_types;
mod searcher;

#[macro_use]
mod macros;

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Args {
    #[clap(subcommand)]
    command: Commands,

    #[clap(flatten)]
    verbose: Verbosity<InfoLevel>,
}

commands_enum!(completion, search);

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Args::parse();

    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .format_module_path(false)
        .format_target(false)
        .init();

    Commands::exec(cli).await?;

    Ok(())
}
