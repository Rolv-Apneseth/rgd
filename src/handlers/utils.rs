use color_eyre::{Result, eyre::eyre};
use lib_game_detector::data::Game;

use crate::{cli::{GameDetailArgs, IdentifyGameArgs}, data::GameField};

/// Find a game matching the given [`IdentifyGameArgs`].
///
/// Returns an error if no match is found.
pub(super) fn find_game(game_id: IdentifyGameArgs) -> Result<Game> {
    let title = game_id.get_title()?;
    let detector = lib_game_detector::get_detector();

    detector
        .get_all_detected_games()
        .into_iter()
        // Match against lowercase titles, for case insensitivity
        .find(|g: &Game| g.title.to_lowercase() == title)
        .ok_or(eyre!("could not find a game with title: \"{title}\""))
}

/// Convert a game to a CSV representation, extracting only the given fields,
/// and using the given delimiter to separate fields.
pub(super) fn game_to_csv<'a>(
    game: &Game,
    delimiter: &str,
    fields: impl IntoIterator<Item = &'a GameField>,
) -> String {
    fields
        .into_iter()
        .map(|f| f.get_from_game(game))
        .collect::<Vec<_>>()
        .join(delimiter)
}

/// Determine what delimiter to used based on the given [`GameDetailArgs`].
pub(super) fn get_delimiter(details: &GameDetailArgs) -> &str {
    details.delimiter.as_deref().unwrap_or("\t")
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_get_delimiter() {
        assert_eq!(
            get_delimiter(&GameDetailArgs {
                delimiter: None,
                ..Default::default()
            }),
            "\t"
        );
        assert_eq!(
            get_delimiter(&GameDetailArgs {
                delimiter: Some("\t".into()),
                ..Default::default()
            }),
            "\t"
        );
        assert_eq!(
            get_delimiter(&GameDetailArgs {
                delimiter: Some("".into()),
                ..Default::default()
            }),
            ""
        );
        assert_eq!(
            get_delimiter(&GameDetailArgs {
                delimiter: Some("\r\n".into()),
                ..Default::default()
            }),
            "\r\n"
        );
        assert_eq!(
            get_delimiter(&GameDetailArgs {
                delimiter: Some("DELIMITER".into()),
                ..Default::default()
            }),
            "DELIMITER"
        );
    }
}
