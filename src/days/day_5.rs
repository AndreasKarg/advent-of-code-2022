use color_eyre::eyre::ContextCompat;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{
    anychar, char, digit1, line_ending, multispace0, newline, one_of, space0,
};
use nom::combinator::{map_res, opt};
use nom::multi::{many0, many1};
use nom::{Finish, IResult};
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {
        "[D]
         [N] [C]
         [Z] [M] [P]
          1   2   3
        "
    };

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
        let (_, actual_stacks) = parse_stacks(INPUT).unwrap();

        // Then
        assert_eq!(actual_stacks, expected_stacks);
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Container {
    identifier: char,
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
    let (res, _) = opt(newline)(res)?;

    Ok((res, column_count))
}

fn parse_stacks(i: &str) -> IResult<&str, Vec<Vec<Container>>> {
    let (res, stack_rows) = many1(parse_row_of_stacks)(i)?;
    let (res, number_of_stacks) = parse_legend_into_column_count(res)?;

    let mut stacks = vec![Vec::default(); number_of_stacks as usize];

    println!("{stack_rows:#?}");

    for row in stack_rows.into_iter().rev() {
        for (column_idx, maybe_container) in row.into_iter().enumerate() {
            if let Some(container) = maybe_container {
                stacks[column_idx].push(container);
            }
        }
    }

    Ok((res, stacks))
}
