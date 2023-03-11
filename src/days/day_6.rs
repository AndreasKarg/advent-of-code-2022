use color_eyre::eyre::eyre;
use color_eyre::Result;
use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;
    use yare::parameterized;

    #[parameterized(
        first =  { "mjqjpqmgbljsphdztnvjfqwrcgsmlb", "7"},
        second = { "bvwbjplbgvbhsrlpgdmjqwftvncz", "5"},
        third = { "nppdvjthqldpwncqszvftbrmjlhg", "6"},
        fourth = { "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", "10"},
        fifth = { "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", "11"},
    )]
    fn part_1_returns_the_number_of_characters_until_the_end_of_the_packet_start_marker(
        input: &str,
        expected: &str,
    ) {
        // Given

        // When
        let solution = solve_part_1(input);

        // Then
        assert_eq!(solution, expected);
    }

    #[parameterized(
    first =  { "mjqjpqmgbljsphdztnvjfqwrcgsmlb", "19"},
    second = { "bvwbjplbgvbhsrlpgdmjqwftvncz", "23"},
    third = { "nppdvjthqldpwncqszvftbrmjlhg", "23"},
    fourth = { "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", "29"},
    fifth = { "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", "26"},
    )]
    fn part_2_returns_the_number_of_characters_until_the_end_of_the_message_start_marker(
        input: &str,
        expected: &str,
    ) {
        // Given

        // When
        let solution = solve_part_2(input);

        // Then
        assert_eq!(solution, expected);
    }
}

pub fn solve_part_1(input_data: &str) -> String {
    let result = solve(input_data, 4).unwrap();

    format!("{result}")
}

pub fn solve_part_2(input_data: &str) -> String {
    let result = solve(input_data, 14).unwrap();

    format!("{result}")
}

fn solve(input_data: &str, window_len: usize) -> Result<usize> {
    for (bytes_before_window, window) in input_data.as_bytes().windows(window_len).enumerate() {
        let mut set: HashSet<u8> = HashSet::new();
        set.extend(window.iter());
        if set.len() == window_len {
            return Ok(bytes_before_window + window_len);
        }
    }

    Err(eyre!("No marker found with length {window_len}!"))
}
