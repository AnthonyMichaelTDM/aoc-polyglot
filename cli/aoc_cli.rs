/// Wrapper module around the "aoc-cli" command-line.
use std::{
    fmt::Display,
    path::PathBuf,
    process::{Command, Output, Stdio},
};

use crate::get_challenge_dir;

#[derive(Debug)]
pub enum AocCommandError {
    CommandNotFound,
    CommandNotCallable,
    BadExitStatus(Output),
    IoError,
}

impl Display for AocCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CommandNotFound => write!(f, "aoc-cli is not present in environment."),
            Self::CommandNotCallable => write!(f, "aoc-cli could not be called."),
            Self::BadExitStatus(_) => {
                write!(f, "aoc-cli exited with a non-zero status.")
            }
            Self::IoError => write!(f, "could not write output files to file system."),
        }
    }
}

/// check if the aoc-cli is callable.
///
/// # Errors
///
/// aoc-cli is not callable or not found.
pub fn check() -> Result<(), AocCommandError> {
    Command::new("aoc")
        .arg("-V")
        .output()
        .map_err(|_| AocCommandError::CommandNotFound)?;
    Ok(())
}

/// use the aoc-cli to read the puzzle description.
///
/// # Errors
///
/// aoc-cli is not callable or not found, or failed to call aoc-cli.
pub fn read(day: u8, year: u16) -> Result<Output, AocCommandError> {
    let puzzle_path = get_puzzle_path(day, year);

    let args = build_args(
        "read",
        &[
            "--description-only".into(),
            "--puzzle-file".into(),
            puzzle_path.to_string_lossy().into_owned(),
        ],
        day,
        year,
    );

    call_aoc_cli(&args)
}

/// use the aoc-cli to download the input and puzzle description.
///
/// # Errors
///
/// aoc-cli is not callable or not found, or failed to call aoc-cli.
pub fn download(day: u8, year: u16) -> Result<Output, AocCommandError> {
    let input_path = get_input_path(day, year);
    let puzzle_path = get_puzzle_path(day, year);

    let args = build_args(
        "download",
        &[
            "--overwrite".into(),
            "--input-file".into(),
            input_path.to_string_lossy().into_owned(),
            "--puzzle-file".into(),
            puzzle_path.to_string_lossy().into_owned(),
        ],
        day,
        year,
    );

    let output = call_aoc_cli(&args)?;
    println!("---");
    println!(
        "🎄 Successfully wrote input to \"{}\".",
        input_path.to_string_lossy()
    );
    println!(
        "🎄 Successfully wrote puzzle to \"{}\".",
        puzzle_path.to_string_lossy()
    );
    Ok(output)
}

/// use the aoc-cli to submit a solution.
///
/// # Errors
///
/// aoc-cli is not callable or not found, or failed to call aoc-cli.
pub fn submit(day: u8, year: u16, part: u8, result: &str) -> Result<Output, AocCommandError> {
    // workaround: the argument order is inverted for submit.
    let mut args = build_args("submit", &[], day, year);
    args.push(part.to_string());
    args.push(result.to_string());
    call_aoc_cli(&args)
}

fn get_input_path(day: u8, year: u16) -> PathBuf {
    let challenge_dir: PathBuf = get_challenge_dir(day, year);
    challenge_dir.join("input.txt")
}

fn get_puzzle_path(day: u8, year: u16) -> PathBuf {
    let challenge_dir: PathBuf = get_challenge_dir(day, year);
    challenge_dir.join("puzzle.md")
}

fn build_args(command: &str, args: &[String], day: u8, year: u16) -> Vec<String> {
    let mut cmd_args = args.to_vec();
    cmd_args.extend(["--year".into(), year.to_string()]);
    cmd_args.extend(["--day".into(), day.to_string(), command.into()]);
    cmd_args
}

fn call_aoc_cli(args: &[String]) -> Result<Output, AocCommandError> {
    // println!("Calling >aoc with: {}", args.join(" "));
    let output = Command::new("aoc")
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|_| AocCommandError::CommandNotCallable)?;

    if output.status.success() {
        Ok(output)
    } else {
        Err(AocCommandError::BadExitStatus(output))
    }
}
