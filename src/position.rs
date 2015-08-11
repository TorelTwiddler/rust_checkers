
/// A position on the board.
#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}

impl Position {
    /// Checks if the other Position is catty-corner to this one
    pub fn is_catty_corner(&self, other: Position) -> bool {
        if (self.row + 1 == other.row || self.row - 1 == other.row) &&
           (self.column + 1 == other.column || self.column - 1 == other.column) {
           return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Position;
    
    #[test]
    fn test_is_catty_corner() {
        let from: Position = Position {row: 1, column: 1};
        assert_eq!(true, from.is_catty_corner(Position {row: 0, column: 0}));
        assert_eq!(true, from.is_catty_corner(Position {row: 2, column: 2}));
        assert_eq!(true, from.is_catty_corner(Position {row: 0, column: 2}));
        assert_eq!(true, from.is_catty_corner(Position {row: 2, column: 0}));
        assert_eq!(false, from.is_catty_corner(Position {row: 0, column: 1}));
        assert_eq!(false, from.is_catty_corner(Position {row: 1, column: 1}));
    }
}
