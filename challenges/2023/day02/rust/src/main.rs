const RED_MAX: u32 = 12;
const GREEN_MAX: u32 = 13;
const BLUE_MAX: u32 = 14;

// sum of the id's of "possible" games
fn part_one(input: &str) -> Option<u32> {
    // each line is formatted as follows:
    // Game <id>: <num> <color>, ... <num> <color>;
    // id: the id of the game, a number
    // num: the number of cubes, a number
    // color: the color of the cubes, either "red", "green", or "blue"
    // sep: the separator between the number and the color, either ";" or ","
    // if it's a ";" then the set terminates and we can move on to the next set
    //
    // the game is "possible" if there are no more than 12 red cubes, 13 green cubes, and 14 blue cubes in any given set
    //
    // the output should be the sum of the id's of all "possible" games
    input
        .split('\n')
        .filter_map(|line| {
            let mut parts = line.split(' ').peekable();
            let _ = parts.next()?; // skip "Game"
            let id = parts.next()?.trim_end_matches(':').parse::<u32>().ok()?;

            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            while let Some(num) = parts.next() {
                let next = parts.next()?;
                let end_of_set = !next.ends_with(',');
                let color = next.trim_end_matches([';', ','].as_ref());
                match color {
                    "red" => red += num.parse::<u32>().ok()?,
                    "green" => green += num.parse::<u32>().ok()?,
                    "blue" => blue += num.parse::<u32>().ok()?,
                    _ => return None,
                }

                if !end_of_set {
                    continue;
                }

                if red > RED_MAX || green > GREEN_MAX || blue > BLUE_MAX {
                    return None;
                } else {
                    red = 0;
                    green = 0;
                    blue = 0;
                    continue;
                }
            }
            Some(id)
        })
        .sum::<u32>()
        .into()
}

fn part_two(input: &str) -> Option<u32> {
    // power of the minimum set of cubes
    input
        .split('\n')
        .filter_map(|line| {
            let mut parts = line.split(' ').peekable();
            let _ = parts.next()?; // skip "Game"
            let id = parts.next()?.trim_end_matches(':').parse::<u32>().ok()?;

            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;

            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            while let Some(num) = parts.next() {
                let next = parts.next()?;
                let end_of_set = !next.ends_with(',');
                let color = next.trim_end_matches([';', ','].as_ref());
                match color {
                    "red" => red += num.parse::<u32>().ok()?,
                    "green" => green += num.parse::<u32>().ok()?,
                    "blue" => blue += num.parse::<u32>().ok()?,
                    _ => return None,
                }

                if !end_of_set {
                    continue;
                }

                (red > max_red).then(|| max_red = red);
                (green > max_green).then(|| max_green = green);
                (blue > max_blue).then(|| max_blue = blue);

                red = 0;
                green = 0;
                blue = 0;
                continue;
            }
            Some((id, (max_red, max_green, max_blue)))
        })
        .map(|(_, (red, green, blue))| red * green * blue)
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = include_str!("../../example1.txt");
        let result = part_one(&input);
        assert_eq!(result, Some(8)); // update this to the correct answer
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("../../example1.txt");
        let result = part_two(&input);
        assert_eq!(result, Some(2286)); // update this to the correct answer
    }
}

// the main function, you do not need to edit this
fn main() {
    // input text
    let input = include_str!("../../input.txt");

    // the single argument is what "part" to run, and must either be "1" or "2"
    match std::env::args().nth(1).as_deref() {
        Some("1") => {
            println!("{}", part_one(&input).unwrap());
        }
        Some("2") => {
            println!("{}", part_two(&input).unwrap());
        }
        _ => {
            eprintln!("Please specify a part to run (1 or 2)");
        }
    }
}
