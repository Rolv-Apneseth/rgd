use color_eyre::eyre::Result;

pub mod get;
pub mod list;
mod utils;

pub trait HandleSubcommand {
    fn handle(self) -> Result<()>;
}
