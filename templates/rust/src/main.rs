fn part_one(_input: &str) -> Option<u32> {
    todo!("your code here");
}

fn part_two(_input: &str) -> Option<u32> {
    todo!("your code here");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = include_str!("../../example.txt");
        let result = part_one(&input);
        assert_eq!(result, None); // update this to the correct answer
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("../../example.txt");
        let result = part_two(&input);
        assert_eq!(result, None); // update this to the correct answer
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
