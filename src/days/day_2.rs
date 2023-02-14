use anyhow::{anyhow, bail, Context, Result};

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn part_one_returns_the_score_of_rock_paper_scissors_with_second_column_is_own_action() {
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

    #[test]
    fn part_two_returns_the_score_of_rock_paper_scissors_with_second_column_as_outcome() {
        // Given
        let input = indoc! {"
            A Y
            B X
            C Z"};

        // When
        let solution = solve_part_2(input);

        // Then
        assert_eq!(solution, "12");
    }
}

pub fn solve_part_1(input: &str) -> String {
    let rows = input.trim().split('\n');
    let games = rows.enumerate().map(|(idx, row)| {
        GamePartOne::try_from(row)
            .with_context(|| format!("Unable to parse row \"{row}\" ({idx})!"))
            .unwrap()
    });

    let scores = games.map(|game| game.score());
    let total_score: i32 = scores.sum();

    total_score.to_string()
}

pub fn solve_part_2(input: &str) -> String {
    let rows = input.trim().split('\n');
    let games = rows.enumerate().map(|(idx, row)| {
        GamePartTwo::try_from(row)
            .with_context(|| format!("Unable to parse row \"{row}\" ({idx})!"))
            .unwrap()
    });

    let scores = games.map(|game| game.score());
    let total_score: i32 = scores.sum();

    total_score.to_string()
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl Shape {
    fn winner_against(self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    fn wins_against(self, other: Self) -> bool {
        self.winner_against() == other
    }

    fn loser_against(self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    fn loses_against(self, other: Self) -> bool {
        self.loser_against() == other
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

impl TryFrom<char> for Outcome {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            'X' => Ok(Outcome::Loss),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            other => Err(anyhow!("Invalid Outcome char {other}!")),
        }
    }
}

struct GamePartOne {
    own: Shape,
    opponent: Shape,
}

impl GamePartOne {
    fn score(&self) -> i32 {
        let own = self.own;
        let opponent = self.opponent;

        let outcome = if own.wins_against(opponent) {
            Outcome::Win
        } else if own.loses_against(opponent) {
            Outcome::Loss
        } else {
            Outcome::Draw
        };

        let score_from_outcome = outcome.score();
        let score_from_shape = self.own.score();

        score_from_outcome + score_from_shape
    }
}

impl TryFrom<&str> for GamePartOne {
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

struct GamePartTwo {
    opponent: Shape,
    outcome: Outcome,
}

impl GamePartTwo {
    fn score(&self) -> i32 {
        let own = match self.outcome {
            Outcome::Win => self.opponent.loser_against(),
            Outcome::Loss => self.opponent.winner_against(),
            Outcome::Draw => self.opponent,
        };

        let score_from_outcome = self.outcome.score();
        let score_from_shape = own.score();

        score_from_outcome + score_from_shape
    }
}

impl TryFrom<&str> for GamePartTwo {
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
        let outcome = game.next().ok_or_else(|| anyhow!("Missing outcome"))?;

        let opponent = Shape::try_from(opponent).context("Invalid opponent shape")?;
        let outcome = Outcome::try_from(outcome).context("Invalid outcome")?;

        Ok(Self { opponent, outcome })
    }
}
