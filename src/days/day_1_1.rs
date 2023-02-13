use std::num::ParseIntError;
use std::str::FromStr;

use anyhow::{Context, Result};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_the_number_of_calories_carried_by_the_elf_with_the_most_calories() {
        // Given
        let input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

        // When
        let solution = solve(input);

        // Then
        assert_eq!(solution, "24000");
    }
}

pub fn solve(input_data: &str) -> String {
    let groups_of_lines = input_data.split("\n\n");
    let number_groups: Result<Vec<_>> = groups_of_lines.map(parse_group).collect();
    let number_groups = number_groups.unwrap();

    let calorie_sums = number_groups
        .into_iter()
        .map(|group| group.into_iter().sum());

    let highest_sum: u32 = calorie_sums.max().unwrap();

    highest_sum.to_string()
}

fn parse_group(group: &str) -> Result<Vec<u32>> {
    let lines = group.split('\n');

    lines
        .enumerate()
        .filter(|(_, line)| !line.is_empty())
        .map(|(idx, line)| {
            u32::from_str(line)
                .with_context(|| format!("Failed to parse \"{line}\" on line {idx}!"))
        })
        .collect()
}
