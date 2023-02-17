use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use std::collections::HashSet;
use std::ops::RangeInclusive;

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {
        "2-4,6-8
         2-3,4-5
         5-7,7-9
         2-8,3-7
         6-6,4-6
         2-6,4-8"
    };

    #[test]
    fn parse_pair_returns_pair_of_ranges() {
        // Given
        let pair = "36-92,35-78";

        // When
        let (first_range, second_range) = parse_pair(pair).unwrap();

        // Then
        assert_eq!(first_range, 36..=92);
        assert_eq!(second_range, 35..=78);
    }

    #[test]
    fn solve_1_returns_the_number_of_fully_overlapping_ranges() {
        // Given
        // INPUT as above

        // When
        let solution = solve_part_1(INPUT.trim());

        // Then
        assert_eq!(solution, "2");
    }

    #[test]
    fn solve_2_returns_the_number_of_all_overlapping_ranges() {
        // Given
        // INPUT as above

        // When
        let solution = solve_part_2(INPUT.trim());

        // Then
        assert_eq!(solution, "4");
    }
}

pub fn solve_part_1(input_data: &str) -> String {
    let groups = parse_input(input_data).unwrap();

    let mut total_fully_overlapping_groups = 0;
    for (first_range, second_range) in groups.iter() {
        if (first_range.start() <= second_range.start())
            && (first_range.end() >= second_range.end())
            || (second_range.start() <= first_range.start())
                && (second_range.end() >= first_range.end())
        {
            total_fully_overlapping_groups += 1;
        }
    }

    total_fully_overlapping_groups.to_string()
}

pub fn solve_part_2(input_data: &str) -> String {
    let groups = parse_input(input_data).unwrap();

    let mut total_overlapping_groups = 0;
    for (first_range, second_range) in groups.iter() {
        if (first_range.start() <= second_range.start())
            && (first_range.end() >= second_range.start())
            || (second_range.start() <= first_range.start())
                && (second_range.end() >= first_range.start())
        {
            total_overlapping_groups += 1;
        }
    }

    total_overlapping_groups.to_string()
}

fn parse_input(input_data: &str) -> Result<Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>> {
    let groups = input_data.trim().split('\n');
    let groups: Result<Vec<_>> = groups.map(parse_pair).collect();
    groups
}

fn parse_range(raw_range: &str) -> Result<RangeInclusive<u32>> {
    let mut halves = raw_range.split('-');
    let first_half = halves
        .next()
        .ok_or_else(|| anyhow! {"Missing first half in {raw_range}!"})?;
    let second_half = halves
        .next()
        .ok_or_else(|| anyhow! {"Missing second half in {raw_range}!"})?;

    let first_half: u32 = first_half.parse()?;
    let second_half: u32 = second_half.parse()?;

    Ok(first_half..=second_half)
}

fn parse_pair(raw_pair: &str) -> Result<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    let halves = raw_pair.split(',');
    let ranges: Result<Vec<_>> = halves.map(parse_range).collect();
    let ranges = ranges.with_context(|| format!("Unable to parse ranges from {raw_pair}!"))?;
    assert_eq!(ranges.len(), 2);

    Ok((ranges[0].clone(), ranges[1].clone()))
}
