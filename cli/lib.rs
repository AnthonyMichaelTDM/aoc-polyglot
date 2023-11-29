use std::{fmt::Display, path::PathBuf, str::FromStr};

use chrono::Datelike;

pub mod aoc_cli;
pub mod commands;
pub mod langs;
pub mod readme_benchmarks;
// pub mod runner;

pub const ANSI_ITALIC: &str = "\x1b[3m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";

#[must_use]
pub fn get_challenge_dir(day: u8, year: u16) -> PathBuf {
    PathBuf::from_iter([
        env!("CARGO_MANIFEST_DIR"),
        &format!("{year}"),
        &format!("day{day:02}"),
    ])
}

// get's the year from the environment or from the current date.
#[must_use]
pub fn get_year() -> u16 {
    get_year_from_env().unwrap_or_else(|| {
        let now = chrono::Local::now();
        if now.month() == 12 {
            u16::try_from(now.year().clamp(0, i32::from(u16::MAX))).unwrap_or_default()
        } else {
            u16::try_from(now.year().clamp(0, i32::from(u16::MAX))).unwrap_or_default() - 1
        }
    })
}

// get's the year from the environment.
fn get_year_from_env() -> Option<u16> {
    std::env::var("AOC_YEAR").ok().and_then(|x| x.parse().ok())
}

#[derive(Clone, Debug)]
#[repr(u8)]
pub enum Part {
    One = 1,
    Two = 2,
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::One => write!(f, "1"),
            Self::Two => write!(f, "2"),
        }
    }
}

impl FromStr for Part {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match u8::from_str(s) {
            Ok(1) => Ok(Self::One),
            Ok(2) => Ok(Self::Two),
            Ok(_) => Err(anyhow::anyhow!("Invalid part")),
            Err(e) => Err(anyhow::anyhow!(e)),
        }
    }
}
