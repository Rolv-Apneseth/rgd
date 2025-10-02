mod cli;
mod data;
mod handlers;
mod logging;
mod utils;

use clap::Parser;
use color_eyre::Result;

use crate::{cli::{Cli, Command}, handlers::HandleSubcommand, logging::init_logging};

fn main() -> Result<()> {
    color_eyre::install()?;

    let _log_guard = init_logging()?;

    tracing::trace!("Parsing args");
    let args = Cli::parse();

    tracing::trace!("Args: {:?}", args);
    let res = match args.command {
        Command::List(cmd) => cmd.handle(),
        Command::Get(cmd) => cmd.handle(),
    };

    res.inspect_err(|e| {
        tracing::error!("{e}");
        let mut source = e.source();
        while let Some(err) = source {
            source = err.source();
            tracing::error!("-> {err}")
        }
    })
}
