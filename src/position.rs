
/// A position on the board, x being the row, y the column.
#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    /// Checks if the other Position is catty-corner to this one
    pub fn is_catty_corner(&self, other: Position) -> bool {
        if (self.x + 1 == other.x || self.x - 1 == other.x) &&
           (self.y + 1 == other.y || self.y - 1 == other.y) {
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
        let from: Position = Position {x: 1, y: 1};
        assert_eq!(true, from.is_catty_corner(Position {x: 0, y: 0}));
        assert_eq!(true, from.is_catty_corner(Position {x: 2, y: 2}));
        assert_eq!(true, from.is_catty_corner(Position {x: 0, y: 2}));
        assert_eq!(true, from.is_catty_corner(Position {x: 2, y: 0}));
        assert_eq!(false, from.is_catty_corner(Position {x: 0, y: 1}));
        assert_eq!(false, from.is_catty_corner(Position {x: 1, y: 1}));
    }
}
