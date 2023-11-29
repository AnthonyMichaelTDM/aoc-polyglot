use crate::{
    aoc_cli,
    langs::{Solution as _, SupportedLanguage},
    Part,
};

/// Handle the `solve` subcommand.
///
/// # Errors
///
/// TODO: Document errors
pub fn handle(
    day: u8,
    year: u16,
    language: SupportedLanguage,
    mut release: bool,
    mut time: bool,
    submit_part: Option<Part>,
) -> anyhow::Result<()> {
    if submit_part.is_some() {
        release = true;
        time = false;
    }

    // compile the solution
    println!("🎄 Compiling solution...");
    language.compile(day, year)?;

    // if we are timing the solution, start the timer
    if time {
        println!("🎄 Timing solution...");
    }
    let start = if time {
        Some(std::time::Instant::now())
    } else {
        None
    };

    // run the solution
    let output = if release {
        language.run(day, year)
    } else {
        language.test(day, year)
    }?;

    // if we are timing the solution, stop the timer
    let end = if time {
        Some(std::time::Instant::now())
    } else {
        None
    };

    // print the output
    {
        let parts = output
            .split("\n\n=================\n\n")
            .collect::<Vec<_>>();

        for (i, result) in parts.iter().enumerate() {
            println!(
                "🎄 Part {part}: {result}",
                part = i + 1,
                result = result.trim()
            );
        }
    }

    // print the elapsed time
    if time {
        let elapsed = end.unwrap().duration_since(start.unwrap());
        println!("🎄 Solution took {}ms", elapsed.as_millis());
    }

    // if we are submitting the solution, parse the output to separate the parts and submit the specified one
    if let Some(submit_part) = submit_part {
        let parts: Vec<&str> = output
            .split("\n\n=================\n\n")
            .collect::<Vec<_>>();

        let part: u8 = submit_part as u8;

        let result: &str = parts.get(part as usize - 1).ok_or_else(|| {
            anyhow::anyhow!("Could not find part {part} in output of day {day:02}, year {year}")
        })?;

        println!("Submitting solution to part {part}...");

        let _ = aoc_cli::submit(day, year, part, result)?;
    }

    Ok(())
}
