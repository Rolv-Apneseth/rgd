use std::io::{Write, stdout};

use clap::Parser;
use color_eyre::{
    Result,
    eyre::{Context, eyre},
};

use crate::{
    cli::GameDetailArgs,
    data::RgdSupportedLaunchers,
    handlers::{
        HandleSubcommand,
        utils::{game_to_csv, get_delimiter},
    },
    utils::ignore_broken_pipe,
};

#[derive(Debug, Parser)]
pub struct Command {
    /// Specify the source (launcher) for which to list games from.
    #[arg(short, long, env = "RGD_LIST_SOURCE")]
    source: Option<RgdSupportedLaunchers>,

    #[command(flatten)]
    details: GameDetailArgs,
}

impl HandleSubcommand for Command {
    fn handle(self) -> Result<()> {
        let Command { source, details } = self;

        let detector = lib_game_detector::get_detector();

        let games = if let Some(source) = source {
            source
                .into_underlying_launchers()
                .into_iter()
                .flat_map(|l| {
                    // Empty list if the source is not detected
                    // TODO: error if source not detected?
                    detector
                        .get_all_detected_games_from_specific_launcher(l)
                        .unwrap_or_default()
                })
                .collect()
        } else {
            detector.get_all_detected_games()
        };

        // Return early if no games were found installed on the user's system
        if games.is_empty() {
            return Err(eyre!("no games found"));
        }

        // Format output - either CSV with a specific delimiter (and no header), or JSON
        let output = if details.json {
            serde_json::to_string_pretty(&games).context("failed to serialize games")?
        } else {
            let delimiter = get_delimiter(&details);
            let fields = &details.fields;
            games
                .into_iter()
                .map(|g| game_to_csv(&g, delimiter, fields))
                .collect::<Vec<_>>()
                .join("\n")
        };

        let mut stdout = stdout().lock();
        ignore_broken_pipe(writeln!(&mut stdout, "{output}",))
            .context("failed to write to STDOUT")?;

        Ok(())
    }
}
