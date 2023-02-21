use anyhow::Context;
use color_eyre::eyre::ContextCompat;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char, digit1, line_ending, multispace0, one_of, space0};
use nom::combinator::{map_res, opt};
use nom::multi::{many0, many1};
use nom::{Finish, IResult};
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parse_row_turns_a_complete_row_into_containers() {
        // Given
        let row_of_containers = "[Z] [M] [P]";
        let expected_containers = vec![
            Some(Container { identifier: 'Z' }),
            Some(Container { identifier: 'M' }),
            Some(Container { identifier: 'P' }),
        ];

        // When
        let (_, actual_containers) = parse_row_of_stacks(row_of_containers).unwrap();

        // Then
        assert_eq!(actual_containers, expected_containers);
    }

    #[test]
    fn parse_row_of_stacks_parses_gaps_as_none() {
        // Given
        let row_of_containers = "[Z]     [P]";
        let expected_containers = vec![
            Some(Container { identifier: 'Z' }),
            None,
            Some(Container { identifier: 'P' }),
        ];

        // When
        let (_, actual_containers) = parse_row_of_stacks(row_of_containers).unwrap();

        // Then
        assert_eq!(actual_containers, expected_containers);
    }

    #[test]
    fn parse_legend_into_column_count_returns_number_of_columns() {
        // Given
        let legend = " 1   2   3";
        let expected_column_count = 3;

        // When
        let (_, actual_column_count) = parse_legend_into_column_count(legend).unwrap();

        // Then
        assert_eq!(actual_column_count, expected_column_count);
    }

    #[test]
    fn parse_stacks_returns_all_stacks_from_the_input() {
        // Given
        let input = indoc! {
            "[D]
             [N] [C]
             [Z] [M] [P]
              1   2   3
            "
        };

        let expected_stacks = vec![
            vec![
                Container { identifier: 'Z' },
                Container { identifier: 'N' },
                Container { identifier: 'D' },
            ],
            vec![Container { identifier: 'M' }, Container { identifier: 'C' }],
            vec![Container { identifier: 'P' }],
        ];

        // When
        let (_, actual_stacks) = parse_stacks(input).unwrap();

        // Then
        assert_eq!(actual_stacks, expected_stacks);
    }

    #[test]
    fn parse_instruction_does_that() {
        // Given
        let input = "move 2 from 3 to 1\n";
        let expected_instruction = Instruction {
            count: 2,
            from: 3,
            to: 1,
        };

        // When
        let (_, actual_instruction) = parse_instruction(input).unwrap();

        // Then
        assert_eq!(actual_instruction, expected_instruction);
    }

    #[test]
    fn parse_input_processes_stacks_and_instructions() {
        // Given
        let input = indoc! {
            "[D]
             [N] [C]
             [Z] [M] [P]
              1   2   3

             move 2 from 3 to 1
             move 17 from 5 to 3
            "
        };
        let expected_puzzle_input = PuzzleInput {
            stacks: vec![
                vec![
                    Container { identifier: 'Z' },
                    Container { identifier: 'N' },
                    Container { identifier: 'D' },
                ],
                vec![Container { identifier: 'M' }, Container { identifier: 'C' }],
                vec![Container { identifier: 'P' }],
            ],

            instructions: vec![
                Instruction {
                    count: 2,
                    from: 3,
                    to: 1,
                },
                Instruction {
                    count: 17,
                    from: 5,
                    to: 3,
                },
            ],
        };

        // When
        let (_, actual_puzzle_input) = parse_input(input).unwrap();

        // Then
        assert_eq!(actual_puzzle_input, expected_puzzle_input);
    }

    #[test]
    fn solve_part_1_executes_instructions_and_returns_character_from_top_of_each_stack() {
        // Given
        let input = indoc! {
            "    [D]
             [N] [C]
             [Z] [M] [P]
              1   2   3

             move 1 from 2 to 1
             move 3 from 1 to 3
             move 2 from 2 to 1
             move 1 from 1 to 2"
        };
        let expected_solution = "CMZ";

        // When
        let actual_solution = solve_part_1(input);

        // Then
        assert_eq!(actual_solution, expected_solution);
    }
}

pub fn solve_part_1(input_data: &str) -> String {
    let (
        _,
        PuzzleInput {
            mut stacks,
            instructions,
        },
    ) = parse_input(input_data).unwrap();

    for instruction in instructions {
        // println!("Current stack state: {stacks:#?}");
        // println!("Executing instruction: {instruction:#?}");
        let stack_from = stacks.get_mut(instruction.from as usize - 1).unwrap();

        let splitting_point = stack_from.len() - instruction.count as usize;
        let mut payload: Vec<_> = stack_from.drain(splitting_point..).collect();
        payload.reverse();

        let stack_to = stacks.get_mut(instruction.to as usize - 1).unwrap();
        stack_to.extend_from_slice(&payload);
    }

    let mut stack_characters = Vec::new();
    for stack in stacks {
        let top_container = stack.last().unwrap();
        stack_characters.push(top_container.identifier);
    }

    stack_characters.into_iter().collect()
}

pub fn solve_part_2(input_data: &str) -> String {
    let (
        _,
        PuzzleInput {
            mut stacks,
            instructions,
        },
    ) = parse_input(input_data).unwrap();

    for instruction in instructions {
        // println!("Current stack state: {stacks:#?}");
        // println!("Executing instruction: {instruction:#?}");
        let stack_from = stacks.get_mut(instruction.from as usize - 1).unwrap();

        let splitting_point = stack_from.len() - instruction.count as usize;
        let mut payload: Vec<_> = stack_from.drain(splitting_point..).collect();

        let stack_to = stacks.get_mut(instruction.to as usize - 1).unwrap();
        stack_to.extend_from_slice(&payload);
    }

    let mut stack_characters = Vec::new();
    for stack in stacks {
        let top_container = stack.last().unwrap();
        stack_characters.push(top_container.identifier);
    }

    stack_characters.into_iter().collect()
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Container {
    identifier: char,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Instruction {
    from: u32,
    to: u32,
    count: u32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct PuzzleInput {
    stacks: Vec<Vec<Container>>,
    instructions: Vec<Instruction>,
}

fn parse_container(i: &str) -> IResult<&str, Option<Container>> {
    let acceptable_identifiers = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let (res, _) = char('[')(i)?;
    let (res, identifier) = one_of(acceptable_identifiers)(res)?;
    let (res, _) = char(']')(res)?;

    Ok((res, Some(Container { identifier })))
}

fn parse_gap(i: &str) -> IResult<&str, Option<Container>> {
    let (res, _) = tag("   ")(i)?;

    Ok((res, None))
}

fn parse_segment(i: &str) -> IResult<&str, Option<Container>> {
    let (res, maybe_container) = alt((parse_container, parse_gap))(i)?;
    let (res, _separator) = opt(char(' '))(res)?;

    Ok((res, maybe_container))
}

fn parse_row_of_stacks(i: &str) -> IResult<&str, Vec<Option<Container>>> {
    let (res, containers) = many1(parse_segment)(i)?;
    let (res, _line_break) = opt(line_ending)(res)?;

    Ok((res, containers))
}

fn parse_legend_into_column_count(i: &str) -> IResult<&str, u32> {
    fn column_label(i: &str) -> IResult<&str, u32> {
        let (res, _whitespace) = space0(i)?;
        nom::character::complete::u32(res)
    }

    let (res, labels) = many1(column_label)(i)?;
    let column_count = labels.into_iter().max().unwrap();

    let (res, _) = space0(res)?;
    let (res, _) = opt(line_ending)(res)?;

    Ok((res, column_count))
}

fn parse_stacks(i: &str) -> IResult<&str, Vec<Vec<Container>>> {
    let (res, stack_rows) = many1(parse_row_of_stacks)(i)?;
    let (res, number_of_stacks) = parse_legend_into_column_count(res)?;

    let mut stacks = vec![Vec::default(); number_of_stacks as usize];

    for row in stack_rows.into_iter().rev() {
        for (column_idx, maybe_container) in row.into_iter().enumerate() {
            if let Some(container) = maybe_container {
                stacks[column_idx].push(container);
            }
        }
    }

    Ok((res, stacks))
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    let (res, _) = tag("move ")(i)?;
    let (res, count) = nom::character::complete::u32(res)?;
    let (res, _) = tag(" from ")(res)?;
    let (res, from) = nom::character::complete::u32(res)?;
    let (res, _) = tag(" to ")(res)?;
    let (res, to) = nom::character::complete::u32(res)?;
    let (res, _) = opt(line_ending)(res)?;

    Ok((res, Instruction { from, to, count }))
}

fn parse_input(i: &str) -> IResult<&str, PuzzleInput> {
    let (res, stacks) = parse_stacks(i)?;
    let (res, _blank_line) = line_ending(res)?;
    let (res, instructions) = many1(parse_instruction)(res)?;

    Ok((
        res,
        PuzzleInput {
            stacks,
            instructions,
        },
    ))
}
