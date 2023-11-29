use std::process::{Command, Stdio};

use crate::{langs::SupportedLanguage, Part};

/// Handle the `solve` subcommand.
///
/// # Errors
///
/// TODO: Document errors
pub fn handle(
    day: u8,
    year: u16,
    language: SupportedLanguage,
    release: bool,
    time: bool,
    submit_part: Option<Part>,
) -> anyhow::Result<()> {
    todo!();

    let day_padded = format!("{day:02}");

    let mut cmd_args = vec!["run".to_string(), "--bin".to_string(), day_padded];

    if release {
        cmd_args.push("--release".to_string());
    }

    cmd_args.push("--".to_string());

    if let Some(submit_part) = submit_part {
        cmd_args.push("--submit".to_string());
        cmd_args.push(submit_part.to_string());
    }

    if time {
        cmd_args.push("--time".to_string());
    }

    let mut cmd = Command::new("cargo")
        .args(&cmd_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    cmd.wait()?;
}
