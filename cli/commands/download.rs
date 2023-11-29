use crate::{aoc_cli, get_challenge_dir};

/// Handle the `download` subcommand.
///
/// # Errors
///
/// aoc-cli is not callable or not found, or failed to call aoc-cli.
pub fn handle(day: u8, year: u16) -> anyhow::Result<()> {
    if aoc_cli::check().is_err() {
        anyhow::bail!("command \"aoc\" not found or not callable. Try running \"cargo install aoc-cli\" to install it.");
    }

    if !get_challenge_dir(day, year).exists() {
        std::fs::create_dir_all(get_challenge_dir(day, year))?;
    }

    if let Err(e) = aoc_cli::download(day, year) {
        anyhow::bail!("failed to call aoc-cli: {e}");
    };

    Ok(())
}
