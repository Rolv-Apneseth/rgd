use std::io::stdout;

use clap::{CommandFactory, Parser};
use clap_complete::aot::{Generator, Shell, generate};
use color_eyre::Result;

use crate::{cli::Cli, handlers::HandleSubcommand};

fn print_completions<G: Generator>(generator: G, cmd: &mut clap::Command) {
    let mut stdout = stdout().lock();
    generate(generator, cmd, cmd.get_name().to_owned(), &mut stdout);
}

#[derive(Debug, Parser)]
pub struct Command {
    /// Shell to generate completions for
    #[arg()]
    pub shell: Shell,
}

impl HandleSubcommand for Command {
    fn handle(self) -> Result<()> {
        print_completions(self.shell, &mut Cli::command());
        Ok(())
    }
}
