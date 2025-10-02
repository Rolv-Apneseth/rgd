use std::path::PathBuf;

/// Ignore broken pipe IO errors.
///
/// See <https://users.rust-lang.org/t/broken-pipe-when-attempt-to-write-to-stdout/111186>
pub fn ignore_broken_pipe(res: std::io::Result<()>) -> std::io::Result<()> {
    match res {
        Err(e) if e.kind() != std::io::ErrorKind::BrokenPipe => Err(e),
        _ => Ok(()),
    }
}

/// Get a displayable value from an optional path.
pub fn display_opt_path(path: Option<&PathBuf>) -> &str {
    path.and_then(|p| p.to_str()).unwrap_or_default()
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_ignore_broken_pipe() {
        use std::io::{Error, ErrorKind};

        use color_eyre::eyre::eyre;

        assert!(ignore_broken_pipe(Err(Error::new(ErrorKind::NotFound, eyre!("")))).is_err());
        assert!(ignore_broken_pipe(Err(Error::new(ErrorKind::AlreadyExists, eyre!("")))).is_err());
        assert!(ignore_broken_pipe(Err(Error::new(ErrorKind::BrokenPipe, eyre!("")))).is_ok());
    }

    #[test]
    fn test_display_opt_path() {
        assert_eq!(display_opt_path(None), "");
        assert_eq!(display_opt_path(Some(&PathBuf::new())), "");
        assert_eq!(display_opt_path(Some(&PathBuf::from("/"))), "/");
        assert_eq!(
            display_opt_path(Some(&PathBuf::from("/path/to/file"))),
            "/path/to/file"
        );
    }
}
