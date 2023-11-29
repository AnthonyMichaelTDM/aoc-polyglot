use tap::Tap as _;

use crate::langs::{Solution, SupportedLanguage};

/// Handle the `scaffold` subcommand.
///
/// # Errors
///
/// errors if the scaffold fails to be created.
pub fn handle(day: u8, year: u16, language: Option<SupportedLanguage>) -> anyhow::Result<()> {
    println!("Creating scaffold for day {day} of year {year}...",);

    match language {
        Some(language) => {
            let _ = language
                .scaffold(day, year)
                .map_err(|e| anyhow::anyhow!("🎄 Failed to create solution scaffold: {e}"))?;
            println!("🎄 Successfully created solution scaffold.");
        }
        None => {
            let rust = SupportedLanguage::Rust
                .scaffold(day, year)
                .tap(|result| match result {
                    Ok(_) => println!("🎄 Successfully created solution scaffold."),
                    Err(e) => eprintln!("🎄 Failed to create solution scaffold: {e}"),
                });
            let python = SupportedLanguage::Python
                .scaffold(day, year)
                .tap(|result| match result {
                    Ok(_) => println!("🎄 Successfully created solution scaffold."),
                    Err(e) => eprintln!("🎄 Failed to create solution scaffold: {e}"),
                });
            let zig = SupportedLanguage::Zig
                .scaffold(day, year)
                .tap(|result| match result {
                    Ok(_) => println!("🎄 Successfully created solution scaffold."),
                    Err(e) => eprintln!("🎄 Failed to create solution scaffold: {e}"),
                });

            if rust.is_err() && python.is_err() && zig.is_err() {
                anyhow::bail!("Failed to create solution scaffold for any language.");
            }
        }
    };

    println!("---");
    println!(
        "🎄 Type `cargo solve {day:02} -y {year} -l {}` to run your solution.",
        language.map_or_else(|| "<language>".to_string(), |l| l.to_string())
    );

    Ok(())
}
