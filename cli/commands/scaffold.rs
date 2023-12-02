use std::{fs::OpenOptions, path};

use anyhow::{bail, Result};
use tap::Tap as _;

use crate::{
    get_challenge_dir,
    langs::{Solution, SupportedLanguage},
};

/// Handle the `scaffold` subcommand.
///
/// # Errors
///
/// errors if the scaffold fails to be created.
pub fn handle(day: u8, year: u16, language: Option<SupportedLanguage>) -> anyhow::Result<()> {
    println!("Creating scaffold for day {day} of year {year}...",);

    if let Some(language) = language {
        language
            .scaffold(day, year)
            .map_err(|e| anyhow::anyhow!("Failed to create solution scaffold: {e}"))?;
        println!("Successfully created solution scaffold.");
    } else {
        let rust = SupportedLanguage::Rust
            .scaffold(day, year)
            .tap(|result| match result {
                Ok(()) => println!("Successfully created solution scaffold."),
                Err(e) => eprintln!("Failed to create solution scaffold: {e}"),
            });
        let python = SupportedLanguage::Python
            .scaffold(day, year)
            .tap(|result| match result {
                Ok(()) => println!("Successfully created solution scaffold."),
                Err(e) => eprintln!("Failed to create solution scaffold: {e}"),
            });
        let zig = SupportedLanguage::Zig
            .scaffold(day, year)
            .tap(|result| match result {
                Ok(()) => println!("Successfully created solution scaffold."),
                Err(e) => eprintln!("Failed to create solution scaffold: {e}"),
            });

        if rust.is_err() && python.is_err() && zig.is_err() {
            anyhow::bail!("Failed to create solution scaffold for any language.");
        }
    }

    safe_create_empty_input_and_example(day, year)?;

    println!("---");
    println!(
        "🎄 Type `cargo solve {day:02} -y {year} -l {}` to run your solution.",
        language.map_or_else(|| "<language>".to_string(), |l| l.to_string())
    );

    Ok(())
}

fn safe_create_empty_input_and_example(day: u8, year: u16) -> Result<()> {
    let input_path = get_challenge_dir(day, year).join(path::Path::new("input.txt"));
    let example_path = get_challenge_dir(day, year).join(path::Path::new("example.txt"));

    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&input_path)
    {
        Ok(_) => {
            println!("\tCreated empty input file \"{}\"", input_path.display());
        }
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            eprintln!(
                "\tInput file \"{}\" already exists, skipping.",
                input_path.display()
            );
        }
        Err(e) => {
            bail!(
                "Failed to create input file \"{}\": {}",
                input_path.display(),
                e
            );
        }
    }

    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&example_path)
    {
        Ok(_) => {
            println!(
                "\tCreated empty example file \"{}\"",
                example_path.display()
            );
        }
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            eprintln!(
                "\tExample file \"{}\" already exists, skipping.",
                example_path.display()
            );
        }
        Err(e) => {
            bail!(
                "Failed to create example file \"{}\": {}",
                example_path.display(),
                e
            );
        }
    }
    Ok(())
}
