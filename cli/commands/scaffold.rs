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
        Some(language) => match language.scaffold(day, year) {
            Ok(_) => println!("🎄 Successfully created solution scaffold."),
            Err(e) => eprintln!("🎄 Failed to create solution scaffold: {e}"),
        },
        None => {
            if SupportedLanguage::iter()
                .map(|language| {
                    language.scaffold(day, year).tap(|result| match result {
                        Ok(_) => println!("🎄 Successfully created solution scaffold."),
                        Err(e) => eprintln!("🎄 Failed to create solution scaffold: {e}"),
                    })
                })
                .all(|result| result.is_err())
            {
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
