#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hiker() {
        // Given
        let input = "What do you get if you multiply six by nine?";

        // When
        let solution = solve(input);

        // Then
        assert_eq!(solution, "42");
    }
}

pub fn solve(input_data: &str) -> String {
    todo!();
}
