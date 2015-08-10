use piece::Piece;
use position::Position;

/// An 8x8 board that is filled with `Option<Piece>`s.
///
/// # Example
/// 
/// ```
/// let mut board: Board = Default::default();
/// ```
pub struct Board {
    places: [[Option<Piece>; 8]; 8],
}

impl Board {

    /// Prints the board in a human-readable format.
    pub fn print(&self) {
        println!("---- Board ----");
        for i in 0 .. 8 {
            for j in 0 .. 8 {
                match self.places[i][j] {
                    Some(Piece {player, .. }) => print!("{:?}", player),
                    None => print!("0"),
                }
            }
            println!(" ");
        }
    }

    /// Initializes the board with new pieces and players
    ///
    /// # Examples
    /// 
    /// ```
    /// let mut board: Board = Default::default();
    /// board.new_game();
    /// ```
    pub fn new_game(&mut self) {
        for j in 0 .. 8 {
            for i in 0 .. 3 {
                if (i + j) % 2 == 0 {
                    self.places[i][j] = Some(Piece {player: 2});
                }
            }
            for i in 5 .. 8 {
                if (i + j) % 2 == 0 {
                    self.places[i][j] = Some(Piece {player: 1});
                }
            }
        }
    }

    /// Moves a piece from `Position from` to `Position to`
    ///
    /// This only checks if there is a piece there or not.
    /// 
    /// # Failures
    /// If there is, the Result is an error.
    ///
    /// # Examples
    /// 
    /// ```
    /// board.new_game();
    /// board.move_piece(Position {x: 0, y: 0}, Position {x: 0, y: 1});
    /// ```
    pub fn move_piece(&mut self, from: Position, to: Position) -> Result<(), &str> {
        if self.valid_move(from, to) {
            let piece: Option<Piece> = self.get_at_position(from);
            self.places[to.x][to.y] = piece; 
            self.places[from.x][from.y] = None;
            return Ok(());
        } else {
            return Err("Invalid move");
        }
    }

    /// Returns `Some(Piece)` at the given position.
    ///
    /// # Examples
    /// 
    /// ```
    /// board.new_game();
    /// board.get_at_position(Position {x: 0, y: 0});
    /// ```
    fn get_at_position(&self, position: Position) -> Option<Piece> {
        self.places[position.x][position.y]
    }
    
    /// Returns true if the move is a valid one (regardless of player)
    ///
    /// This will check for:
    /// `to` is empty
    /// one of:
    ///     (`to` is catty-corner to `from`) or
    ///     (`to` is double catty-corner to `from` and
    ///      there is a piece of the other player between `to` and `from`)
    /// 
    /// # Examples
    ///
    /// ```
    /// let mut board: Board = Default::default();
    /// board.new_game();
    /// assert_eq!(false, board.valid_move(Position {x: 0, y: 0},
    ///                                    Position {x: 1, y: 0}))
    /// assert_eq!(true, board.valid_move(Position {x: 0, y: 2},
    ///                                   Position {x: 1, y: 3}))
    fn valid_move(&self, from: Position, to: Position) -> bool {
        let piece: Option<Piece> = self.get_at_position(from);
        //TODO: this function isn't working correctly.
        if piece.is_some() {
            return false;
        }
        // Catty-corner check
        if !from.is_catty_corner(to) {
            return false;
        }
        return true;
    }
}

/// This fills out the board by default, so we don't have
/// to when initializing.
impl Default for Board {
    fn default() -> Board {
        Board {
            places: [[None; 8]; 8],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Board;
    use position::Position;
    
    #[test]
    fn test_valid_move() {
        let mut board: Board = Default::default();
        board.new_game();
        println!("{:?}", board.places[0][2]);
        println!("{:?}", board.places[1][3]);
        println!("{:?}", board.places[2][0]);
        println!("{:?}", board.places[3][1]);
        assert_eq!(true, board.valid_move(
            Position {x: 0, y: 2}, Position {x: 1, y: 3}));
    }
}
