fn part_one(input: &str) -> Option<u32> {
    todo!("your code here");
}

fn part_two(input: &str) -> Option<u32> {
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
    // include macro
    let input = include_str!("../../input.txt");

    // run the parts
    println!("{}\n", part_one(&input).unwrap_or_default());
    println!("{}", part_two(&input).unwrap_or_default());

    run_part(
        part_one,
        &input,
        {
            {
                DAY
            }
        },
        1,
    );
    run_part(
        part_two,
        &input,
        {
            {
                DAY
            }
        },
        2,
    );
}
