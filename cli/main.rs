use advent_of_code::langs::SupportedLanguage;
use advent_of_code::{commands, get_year, Part};

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "Advent of Code CLI",
    about = "A CLI for managing Advent of Code solutions across multiple languages and years.",
    version = "0.1.0",
    author = "Anthony Rubick"
)]
pub enum AppArguments {
    #[command(about = "Download the challenge for the day and year specified")]
    Download {
        day: u8,
        #[arg(short, long)]
        year: Option<u16>,
    },
    #[command(about = "Read description of the challenge for the day and year specified")]
    Read {
        day: u8,
        #[arg(short, long)]
        year: Option<u16>,
    },
    #[command(about = "Generate a solution template for the specified programming language")]
    Scaffold {
        day: u8,
        #[arg(short, long)]
        year: Option<u16>,
        #[arg(
            short,
            long,
            help = "language to scaffold solution for, leave blank to scaffold for all languages"
        )]
        language: Option<SupportedLanguage>,
    },
    Solve {
        day: u8,
        #[arg(short, long)]
        year: Option<u16>,
        #[arg(
            long,
            help = "whether to use real inputs or not (set to false to run tests)"
        )]
        release: bool,
        #[arg(long, help = "whether to time the solution or not")]
        time: bool,
        #[arg(short, long, help = "language to run solution of")]
        language: SupportedLanguage,
        #[arg(
            short,
            long,
            help = "use aoc_cli to submit the solution, runs with real inputs and does not time the solution"
        )]
        submit: Option<Part>,
    },
    All {
        #[arg(short, long)]
        year: Option<u16>,
        #[arg(
            long,
            help = "whether to use real inputs or not (set to false to run tests)"
        )]
        release: bool,
        #[arg(
            long,
            help = "whether to time the solution or not, runs with real inputs and does not time the solution"
        )]
        time: bool,
        #[arg(
            short,
            long,
            help = "specify to only run the solutions of a specific language"
        )]
        language: Option<SupportedLanguage>,
    },
}

fn main() -> Result<()> {
    match AppArguments::parse() {
        AppArguments::All {
            year,
            release,
            time,
            language,
        } => commands::all::handle(year.unwrap_or_else(|| get_year()), language, release, time)?,
        AppArguments::Download { day, year } => {
            commands::download::handle(day, year.unwrap_or_else(|| get_year()))?
        }
        AppArguments::Read { day, year } => {
            commands::read::handle(day, year.unwrap_or_else(|| get_year()))?
        }
        AppArguments::Scaffold {
            day,
            year,
            language,
        } => commands::scaffold::handle(day, year.unwrap_or_else(|| get_year()), language)?,
        AppArguments::Solve {
            day,
            year,
            release,
            time,
            submit,
            language,
        } => commands::solve::handle(
            day,
            year.unwrap_or_else(|| get_year()),
            language,
            release,
            time,
            submit,
        )?,
    }

    Ok(())
}
