use std::io::{Write, stdout};

use clap::Parser;
use color_eyre::{Result, eyre::Context};

use crate::{
    cli::{GameDetailArgs, IdentifyGameArgs},
    handlers::{
        HandleSubcommand,
        utils::{find_game, game_to_csv, get_delimiter},
    },
    utils::ignore_broken_pipe,
};

#[derive(Debug, Parser)]
pub struct Command {
    #[command(flatten)]
    game_id: IdentifyGameArgs,

    #[command(flatten)]
    details: GameDetailArgs,
}

impl HandleSubcommand for Command {
    fn handle(self) -> Result<()> {
        let Command { game_id, details } = self;

        let game = find_game(game_id)?;

        // Format output - either CSV with a specific delimiter (and no header), or JSON
        let output = if details.json {
            serde_json::to_string_pretty(&game).context("failed to serialize games")?
        } else {
            let delimiter = get_delimiter(&details);
            let fields = &details.fields;
            game_to_csv(&game, delimiter, fields)
        };

        let mut stdout = stdout().lock();
        ignore_broken_pipe(writeln!(&mut stdout, "{output}",))
            .context("failed to write to STDOUT")?;

        Ok(())
    }
}
