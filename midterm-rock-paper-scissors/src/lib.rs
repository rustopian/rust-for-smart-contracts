pub fn determine_winner(p1: Move, p2: Move) -> Outcome {
    // TODO: Implement the game logic using pattern matching
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rock_beats_scissors() {
        let p1 = Move::Rock;
        let p2 = Move::Scissors;
        assert_eq!(determine_winner(p1, p2), Outcome::Win);
        assert_eq!(determine_winner(p2, p1), Outcome::Lose);
    }

    #[test]
    fn test_scissors_beats_paper() {
        let p1 = Move::Scissors;
        let p2 = Move::Paper;
        assert_eq!(determine_winner(p1, p2), Outcome::Win);
        assert_eq!(determine_winner(p2, p1), Outcome::Lose);
    }

    #[test]
    fn test_paper_beats_rock() {
        let p1 = Move::Paper;
        let p2 = Move::Rock;
        assert_eq!(determine_winner(p1, p2), Outcome::Win);
        assert_eq!(determine_winner(p2, p1), Outcome::Lose);
    }

    #[test]
    fn test_tie() {
        let moves = vec![Move::Rock, Move::Paper, Move::Scissors];
        for m in moves {
            assert_eq!(determine_winner(m, m), Outcome::Tie);
        }
    }
}