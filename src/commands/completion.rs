use super::*;

use clap::CommandFactory;
use clap_complete::{generate, Shell};
use std::io;

/// Generate completion script
#[derive(Parser)]
pub struct Args {
    shell: Shell,
}

pub async fn command(args: Args) -> Result<()> {
    generate(
        args.shell,
        &mut crate::Args::command(),
        "member-onboard",
        &mut io::stdout(),
    );
    Ok(())
}
