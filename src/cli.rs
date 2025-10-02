use std::io::{Read, stdin};

use clap::{Parser, Subcommand, command};
use color_eyre::eyre::Context;

use crate::{
    data::GameField,
    handlers::{get, list},
};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// List details of detected games and custom entries.
    #[command()]
    List(list::Command),

    /// Get details of a specific game or custom entry.
    ///
    /// Note that this will still scan all games on your system, but only return
    /// the specified entry if found.
    #[command()]
    Get(get::Command),
}

/// Argument(s) used to identify a specific game / custom entry.
#[derive(Debug, clap::Args)]
pub struct IdentifyGameArgs {
    /// Title of the game as displayed by the list command.
    ///
    /// Note: the matching is case-insensitive, and the first game
    /// with a matching title will be returned. Be aware of having
    /// multiple copies of the same game from different sources.
    #[arg()]
    pub game_title: Option<String>,
}

impl IdentifyGameArgs {
    #[inline]
    pub fn get_title(self) -> color_eyre::Result<String> {
        // Parse from STDIN if not given as an argument
        let title = if self.game_title.as_ref().is_none_or(String::is_empty) {
            let mut t = String::new();
            let mut stdin = stdin().lock();
            stdin
                .read_to_string(&mut t)
                .context("failed to read game title from STDIN")?;
            t
        }
        // Otherwise just take the title passed as an argument
        else {
            self.game_title.expect("title should exist at this stage")
        };

        // Lowercase as matches need to be case-insensitive
        Ok(title.trim().to_lowercase())
    }
}

/// Arguments used to define how game details should be output.
#[derive(Debug, Default, clap::Args)]
pub struct GameDetailArgs {
    /// Output details in JSON format.
    ///
    /// Incompatible with choosing specific fields - all game details will be
    /// output.
    #[arg(short, long, action, conflicts_with = "delimiter")]
    pub json: bool,

    /// Delimiter used to separate detail columns (Default: \t).
    #[arg(short, long, conflicts_with = "json")]
    pub delimiter: Option<String>,

    /// Fields (details about the game) which will be included in the final
    /// output.
    #[arg(short, long, conflicts_with = "json", default_values = ["title", "launch-command"],
        value_delimiter = ',', num_args = 1..)]
    pub fields: Vec<GameField>,
}
