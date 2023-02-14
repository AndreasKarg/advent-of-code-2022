use anyhow::{anyhow, bail, Context, Result};

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn returns_the_score_of_rock_paper_scissors() {
        // Given
        let input = indoc! {"
            A Y
            B X
            C Z"};

        // When
        let solution = solve_part_1(input);

        // Then
        assert_eq!(solution, "15");
    }
}

enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

impl TryFrom<char> for Shape {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            'A' | 'X' => Ok(Shape::Rock),
            'B' | 'Y' => Ok(Shape::Paper),
            'C' | 'Z' => Ok(Shape::Scissors),
            other => Err(anyhow!("Invalid shape char {other}!")),
        }
    }
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn score(&self) -> i32 {
        match self {
            Outcome::Win => 6,
            Outcome::Loss => 0,
            Outcome::Draw => 3,
        }
    }
}

struct Game {
    own: Shape,
    opponent: Shape,
}

impl Game {
    fn score(&self) -> i32 {
        let outcome = match (&self.own, &self.opponent) {
            (Shape::Rock, Shape::Scissors)
            | (Shape::Paper, Shape::Rock)
            | (Shape::Scissors, Shape::Paper) => Outcome::Win,
            (Shape::Rock, Shape::Paper)
            | (Shape::Paper, Shape::Scissors)
            | (Shape::Scissors, Shape::Rock) => Outcome::Loss,
            _ => Outcome::Draw,
        };

        let score_from_outcome = outcome.score();
        let score_from_shape = self.own.score();

        score_from_outcome + score_from_shape
    }
}

impl TryFrom<&str> for Game {
    type Error = anyhow::Error;

    fn try_from(game: &str) -> Result<Self> {
        let mut game = game.chars();
        let opponent = game
            .next()
            .ok_or_else(|| anyhow!("Missing opponent action"))?;

        let space = game.next().ok_or_else(|| anyhow!("Missing space"))?;
        if space != ' ' {
            bail!("No space where there should be one");
        }
        let own = game.next().ok_or_else(|| anyhow!("Missing own action"))?;

        let opponent = Shape::try_from(opponent).context("Invalid opponent shape")?;
        let own = Shape::try_from(own).context("Invalid own shape")?;

        Ok(Self { own, opponent })
    }
}

pub fn solve_part_1(input: &str) -> String {
    let rows = input.trim().split('\n');
    let games = rows.enumerate().map(|(idx, row)| {
        Game::try_from(row)
            .with_context(|| format!("Unable to parse row \"{row}\" ({idx})!"))
            .unwrap()
    });

    let scores = games.map(|game| game.score());
    let total_score: i32 = scores.sum();

    total_score.to_string()
}
