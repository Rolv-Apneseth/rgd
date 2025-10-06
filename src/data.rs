use std::{borrow::Cow, fmt::Display};

use clap::ValueEnum;
use lib_game_detector::data::{Game, SupportedLaunchers};

use crate::utils::display_opt_path;

#[derive(Debug, Clone, ValueEnum)]
pub enum GameField {
    Title,
    PathIcon,
    PathBoxArt,
    PathGameDir,
    LaunchCommand,
    Source,
}

impl GameField {
    pub fn get_from_game<'a>(&self, game: &'a Game) -> Cow<'a, str> {
        match self {
            GameField::Title => (&game.title).into(),
            GameField::PathIcon => display_opt_path(game.path_icon.as_ref()).into(),
            GameField::PathBoxArt => display_opt_path(game.path_box_art.as_ref()).into(),
            GameField::PathGameDir => display_opt_path(game.path_game_dir.as_ref()).into(),
            GameField::LaunchCommand => (format!("{:?}", game.launch_command)).into(),
            GameField::Source => game.source.to_string().into(),
        }
    }
}

#[derive(Debug, Clone, ValueEnum, PartialEq, Eq)]
pub enum RgdSupportedLaunchers {
    Steam,
    Lutris,
    Bottles,
    Heroic,
    Prism,
    AT,
}

impl Display for RgdSupportedLaunchers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RgdSupportedLaunchers::Steam => "Steam",
                RgdSupportedLaunchers::Lutris => "Lutris",
                RgdSupportedLaunchers::Bottles => "Bottles",
                RgdSupportedLaunchers::Heroic => "Heroic Games Launcher",
                RgdSupportedLaunchers::Prism => "Prism Launcher",
                RgdSupportedLaunchers::AT => "ATLauncher",
            }
        )
    }
}

impl RgdSupportedLaunchers {
    pub fn into_underlying_launchers(self) -> Vec<SupportedLaunchers> {
        match self {
            RgdSupportedLaunchers::Steam => vec![
                SupportedLaunchers::Steam,
                SupportedLaunchers::SteamShortcuts,
            ],
            RgdSupportedLaunchers::Lutris => vec![SupportedLaunchers::Lutris],
            RgdSupportedLaunchers::Bottles => vec![SupportedLaunchers::Bottles],
            RgdSupportedLaunchers::Heroic => vec![
                SupportedLaunchers::HeroicGamesAmazon,
                SupportedLaunchers::HeroicGamesEpic,
                SupportedLaunchers::HeroicGamesGOG,
                SupportedLaunchers::HeroicGamesSideload,
            ],
            RgdSupportedLaunchers::Prism => vec![SupportedLaunchers::MinecraftPrism],
            RgdSupportedLaunchers::AT => vec![SupportedLaunchers::MinecraftAT],
        }
    }
}

impl From<SupportedLaunchers> for RgdSupportedLaunchers {
    fn from(value: SupportedLaunchers) -> Self {
        match value {
            SupportedLaunchers::Steam => RgdSupportedLaunchers::Steam,
            SupportedLaunchers::SteamShortcuts => RgdSupportedLaunchers::Steam,
            SupportedLaunchers::Lutris => RgdSupportedLaunchers::Lutris,
            SupportedLaunchers::Bottles => RgdSupportedLaunchers::Bottles,
            SupportedLaunchers::HeroicGamesAmazon => RgdSupportedLaunchers::Heroic,
            SupportedLaunchers::HeroicGamesEpic => RgdSupportedLaunchers::Heroic,
            SupportedLaunchers::HeroicGamesGOG => RgdSupportedLaunchers::Heroic,
            SupportedLaunchers::HeroicGamesSideload => RgdSupportedLaunchers::Heroic,
            SupportedLaunchers::MinecraftPrism => RgdSupportedLaunchers::Prism,
            SupportedLaunchers::MinecraftAT => RgdSupportedLaunchers::AT,
        }
    }
}

#[cfg(test)]
mod test {
    use std::{path::PathBuf, process::Command};

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn gamefield_from_game() {
        let game = Game {
            title: String::from("test"),
            path_icon: None,
            path_box_art: Some(PathBuf::from("/home/rolv")),
            path_game_dir: Some(PathBuf::from("/")),
            launch_command: Command::new("ls"),
            source: SupportedLaunchers::Steam,
        };
        assert_eq!(GameField::Title.get_from_game(&game), String::from("test"));
        assert_eq!(GameField::PathIcon.get_from_game(&game), "");
        assert_eq!(GameField::PathBoxArt.get_from_game(&game), "/home/rolv");
        assert_eq!(GameField::PathGameDir.get_from_game(&game), "/");
        assert_eq!(GameField::LaunchCommand.get_from_game(&game), "\"ls\"");
        assert_eq!(GameField::Source.get_from_game(&game), "Steam");

        let game = Game {
            title: String::from("test2"),
            path_icon: None,
            path_box_art: Some(PathBuf::from("/")),
            path_game_dir: Some(PathBuf::from("/home/user")),
            launch_command: Command::new("echo"),
            source: SupportedLaunchers::Lutris,
        };
        assert_eq!(GameField::Title.get_from_game(&game), String::from("test2"));
        assert_eq!(GameField::PathIcon.get_from_game(&game), "");
        assert_eq!(GameField::PathBoxArt.get_from_game(&game), "/");
        assert_eq!(GameField::PathGameDir.get_from_game(&game), "/home/user");
        assert_eq!(GameField::LaunchCommand.get_from_game(&game), "\"echo\"");
        assert_eq!(GameField::Source.get_from_game(&game), "Lutris");
    }
}
