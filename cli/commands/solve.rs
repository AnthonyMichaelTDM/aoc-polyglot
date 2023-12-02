use time::Duration;

use crate::{
    aoc_cli,
    langs::{Solution as _, SupportedLanguage},
    Part,
};

/// Handle the `solve` subcommand.
///
/// # Errors
///
/// if the solution fails to compile or run, the solution directory does not exist, or the solution can't be submitted
pub fn handle(
    day: u8,
    year: u16,
    language: SupportedLanguage,
    time: bool,
    submit_part: Option<Part>,
) -> anyhow::Result<()> {
    // compile the solution
    println!("🎄 Compiling solution...");
    language.compile(day, year)?;

    // run the solutions
    let (part1, part2) = run_solution(day, year, language, time)?;

    // if we are submitting the solution, parse the output to separate the parts and submit the specified one
    if let Some(submit_part) = submit_part {
        match submit_part {
            Part::One => {
                println!("Submitting solution to part 1...");
                let _ = aoc_cli::submit(day, year, 1, &part1)?;
            }
            Part::Two => {
                println!("Submitting solution to part 2...");
                let _ = aoc_cli::submit(day, year, 2, &part2)?;
            }
        }
    }

    Ok(())
}

fn run_solution(
    day: u8,
    year: u16,
    language: SupportedLanguage,
    time: bool,
) -> anyhow::Result<(String, String)> {
    // compile the solution
    // println!("🎄 Compiling solution...");
    language.compile(day, year)?;

    // time and run part 1
    let part_1_ellapsed;
    let part_1_output;
    {
        let start = time::Instant::now();
        let output = language.run_part(day, year, Part::One).unwrap_or_default();
        // let output = language.run_part(day, year, Part::One)?;
        part_1_ellapsed = start.elapsed();
        part_1_output = output;
    }

    // time and run part 2
    let part_2_ellapsed;
    let part_2_output;
    {
        let start = time::Instant::now();
        let output = language.run_part(day, year, Part::Two).unwrap_or_default();
        // let output = language.run_part(day, year, Part::Two)?;
        part_2_ellapsed = start.elapsed();
        part_2_output = output;
    }

    // if we are timing the solution, benchmark it a number of times (dependent on how long it took to run)
    if time {
        // run a number of times based on how long it took to run the first time, between 10 and 1000 times
        #[allow(
            clippy::cast_sign_loss,
            clippy::cast_precision_loss,
            clippy::cast_possible_truncation
        )]
        let num_runs = (1_000.0 / part_1_ellapsed.whole_milliseconds() as f64).max(10.0) as usize;

        // time and run part 1
        let part_1_ellapsed;
        {
            let start = time::Instant::now();
            for _ in 0..num_runs {
                let _ = language.run_part(day, year, Part::One)?;
            }
            part_1_ellapsed = start.elapsed();
        }

        // time and run part 2
        let part_2_ellapsed;
        {
            let start = time::Instant::now();
            for _ in 0..num_runs {
                let _ = language.run_part(day, year, Part::Two)?;
            }
            part_2_ellapsed = start.elapsed();
        }

        // calculate the average time per run
        let part_1_average = part_1_ellapsed / u32::try_from(num_runs).unwrap();
        let part_2_average = part_2_ellapsed / u32::try_from(num_runs).unwrap();

        // print the output
        println!(
            "Part 1: {part_1_output} ({time} average)",
            time = fmt_duration(part_1_average),
        );
        println!(
            "Part 2: {part_2_output} ({time} average)",
            time = fmt_duration(part_2_average),
        );
    } else {
        // print the output
        println!(
            "Part 1: {part_1_output} ({time})",
            time = fmt_duration(part_1_ellapsed),
        );
        println!(
            "Part 2: {part_2_output} ({time})",
            time = fmt_duration(part_2_ellapsed),
        );
    }
    Ok((part_1_output, part_2_output))
}

fn fmt_duration(duration: Duration) -> String {
    type ChunkUnits = &'static str;
    type ChunkExtractor = fn(Duration) -> i128;
    let time_unit_extractors: &[(ChunkUnits, ChunkExtractor)] = &[
        ("d", |d| i128::from(d.whole_days())),
        ("h", |d| i128::from(d.whole_hours() % 24)),
        ("m", |d| i128::from(d.whole_minutes() % 60)),
        ("s", |d| i128::from(d.whole_seconds() % 60)),
        ("ms", |d| d.whole_milliseconds() % 1000),
        ("μs", |d| d.whole_microseconds() % 1000),
        ("ns", |d| d.whole_nanoseconds() % 1000),
    ];

    let timings: Vec<String> = time_unit_extractors
        .iter()
        .filter_map(|(unit, extract_time_unit)| {
            let value = extract_time_unit(duration);
            if value > 0 {
                Some(format!("{value}{unit}"))
            } else {
                None
            }
        })
        .take(2)
        .collect();

    timings.join(" ").trim().to_string()
}
