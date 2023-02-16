use anyhow::{Context, Result};
use itertools::Itertools;
use std::collections::HashSet;
use std::hash::Hash;

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
