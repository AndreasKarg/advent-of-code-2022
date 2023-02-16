use anyhow::Result;
use itertools::Itertools;
use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn find_fault_returns_the_only_character_that_occurs_in_both_compartment() {
        // Given
        let rucksack = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";

        // When
        let fault = find_fault(rucksack).unwrap();

        // Then
        assert_eq!(fault, 'L');
    }

    #[test]
    fn items_have_a_priority_matching_their_ascii_values() {
        // Given
        let items = "pLPvts".chars();

        // When
        let priorities: Vec<_> = items.map(score_item).collect();

        // Then
        assert_eq!(priorities.iter().sum::<u32>(), 157);
    }

    #[test]
    fn solve_1_returns_the_sum_of_priorities_for_all_faulty_items() {
        // Given
        let input = indoc! {
            "vJrwpWtwJgWrhcsFMMfFFhFp
             jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
             PmmdzqPrVvPwwTWBwg
             wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
             ttgJtRGJQctTZtZT
             CrZsJsPPZsGzwwsLwLmpwMDw"
        }
        .trim();

        // When
        let solution = solve_part_1(input);

        // Then
        assert_eq!(solution, "157");
    }

    #[test]
    fn find_badge_returns_the_item_that_is_common_for_all_elves_in_the_group() {
        // Given
        let items = [
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
        ];

        // When
        let badge = find_badge(&items).unwrap();

        // Then
        assert_eq!(badge, 'r');
    }

    #[test]
    fn solve_2_returns_the_sum_of_priorities_for_all_group_badges() {
        // Given
        let input = indoc! {
            "vJrwpWtwJgWrhcsFMMfFFhFp
             jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
             PmmdzqPrVvPwwTWBwg
             wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
             ttgJtRGJQctTZtZT
             CrZsJsPPZsGzwwsLwLmpwMDw"
        }
        .trim();

        // When
        let solution = solve_part_2(input);

        // Then
        assert_eq!(solution, "70");
    }
}

pub fn solve_part_1(input_data: &str) -> String {
    let rucksacks = input_data.trim().split('\n');
    let mut total_score = 0;
    for rucksack in rucksacks {
        let fault = find_fault(rucksack).unwrap();
        let score = score_item(fault);

        total_score += score;
    }

    total_score.to_string()
}

pub fn solve_part_2(input_data: &str) -> String {
    let rucksacks = input_data.trim().split('\n');
    let groups = rucksacks.array_chunks::<3>();

    let mut total_score = 0;
    for group in groups {
        let badge = find_badge(&group).unwrap();
        let score = score_item(badge);

        total_score += score;
    }

    total_score.to_string()
}

fn score_item(item: char) -> u32 {
    let lowercase_item = item.to_ascii_lowercase();
    let lowercase_priority = lowercase_item as u32 - 'a' as u32 + 1;

    if item.is_ascii_uppercase() {
        lowercase_priority + 26
    } else {
        lowercase_priority
    }
}

fn find_fault(rucksack: &str) -> Result<char> {
    let (first_compartment, second_compartment) = rucksack.split_at(rucksack.len() / 2);

    let item_types_in_first_compartment: HashSet<_> = first_compartment.chars().collect();
    let item_types_in_second_compartment: HashSet<_> = second_compartment.chars().collect();

    let item_types_in_both_compartments: Vec<_> = item_types_in_first_compartment
        .intersection(&item_types_in_second_compartment)
        .cloned()
        .collect();

    let only_overlapping_item = item_types_in_both_compartments.into_iter().exactly_one()?;

    Ok(only_overlapping_item)
}

fn find_badge(rucksacks: &[&str; 3]) -> Result<char> {
    let item_types = rucksacks.map(|rucksack| rucksack.chars().collect::<HashSet<_>>());
    let first_two_common_items: HashSet<_> = item_types[0]
        .intersection(&item_types[1])
        .cloned()
        .collect();
    let possible_badges: Vec<_> = first_two_common_items
        .intersection(&item_types[2])
        .cloned()
        .collect();
    let badge = possible_badges.into_iter().exactly_one()?;

    Ok(badge)
}
