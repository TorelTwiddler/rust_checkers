use piece::Piece;
use position::Position;

#[derive(Debug, PartialEq)]
enum MoveError {IsNotCattyCorner, BlockedByPiece}


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
    /// board.move_piece(Position {row: 0, column: 0}, Position {row: 0, column: 1});
    /// ```
    pub fn move_piece(&mut self, from: Position, to: Position) -> Result<(), MoveError> {
        match self.valid_move(from, to) {
            Ok(_) => {
                let piece: Option<Piece> = self.get_at_position(from);
                self.places[to.row][to.column] = piece; 
                self.places[from.row][from.column] = None;
                return Ok(());
                }
            Err(e) => {
                return Err(e);
            }
        }
    }

    /// Returns `Some(Piece)` at the given position.
    ///
    /// # Examples
    /// 
    /// ```
    /// board.new_game();
    /// board.get_at_position(Position {row: 0, column: 0});
    /// ```
    fn get_at_position(&self, position: Position) -> Option<Piece> {
        self.places[position.row][position.column]
    }
    
    /// Returns a Result<(), MoveError> if the move is a valid one 
    ///     (regardless of player)
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
    /// assert_eq!(false, board.valid_move(Position {row: 0, column: 0},
    ///                                    Position {row: 1, column: 0}))
    /// assert_eq!(true, board.valid_move(Position {row: 0, column: 2},
    ///                                   Position {row: 1, column: 3}))
    fn valid_move(&self, from: Position, to: Position) -> Result<(), MoveError> {
        let to_piece: Option<Piece> = self.get_at_position(to);
        //TODO: this function isn't working correctly.
        if to_piece.is_some() {
            return Err(MoveError::BlockedByPiece);
        }
        // Catty-corner check
        if !from.is_catty_corner(to) {
            return Err(MoveError::IsNotCattyCorner);
        }
        return Ok(());
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
    use super::MoveError;
    use position::Position;
    
    #[test]
    fn test_valid_move() {
        let mut board: Board = Default::default();
        board.new_game();
        println!("{:?}", board.places[2][0]);
        println!("{:?}", board.places[3][1]);
        assert_eq!(Ok(()), board.valid_move(Position {row: 2, column: 0}, 
                                            Position {row: 3, column: 1}));
        assert_eq!(Err(MoveError::BlockedByPiece), 
                   board.valid_move(Position {row: 1, column: 1}, 
                                    Position {row: 2, column: 2}));
        assert_eq!(Err(MoveError::IsNotCattyCorner), 
                   board.valid_move(Position {row: 1, column: 1}, 
                                    Position {row: 3, column: 2}));
    }
}
