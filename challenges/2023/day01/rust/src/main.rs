/// for each line, find the first and last numeric character, and add them together to form a number
/// then sum all of those numbers together
fn part_one(input: &str) -> u32 {
    input
        .split('\n')
        .filter_map(|line| {
            let mut iter = line.chars().filter(|c| c.is_numeric());
            match (iter.next(), iter.last()) {
                (Some(first), None) => format!("{first}{first}").parse::<u32>().ok(),
                (Some(first), Some(last)) => format!("{first}{last}").parse::<u32>().ok(),
                _ => None,
            }
        })
        .sum()
}
/// same as part one, but also consider words that represent numbers ("one" through "nine")
fn part_two(input: &str) -> u32 {
    // idea, replace number words with numbers (only need first and last), then use part one
    input
        .split('\n')
        .map(|line| {
            line.replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e")
        })
        .filter_map(|line| {
            let mut iter = line.chars().filter(|c| c.is_numeric());
            match (iter.next(), iter.last()) {
                (Some(first), None) => format!("{first}{first}").parse::<u32>().ok(),
                (Some(first), Some(last)) => format!("{first}{last}").parse::<u32>().ok(),
                _ => None,
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = include_str!("../../example1.txt");
        let result = part_one(&input);
        assert_eq!(result, 142); // update this to the correct answer
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("../../example2.txt");
        let result = part_two(&input);
        assert_eq!(result, 281); // update this to the correct answer
    }
}

// the main function, you do not need to edit this
fn main() {
    // input text
    let input = include_str!("../../input.txt");

    // the single argument is what "part" to run, and must either be "1" or "2"
    match std::env::args().nth(1).as_deref() {
        Some("1") => {
            println!("{}", part_one(&input));
        }
        Some("2") => {
            println!("{}", part_two(&input));
        }
        _ => {
            eprintln!("Please specify a part to run (1 or 2)");
        }
    }
}
