use piece::Piece;
use position::Position;

#[derive(Debug, PartialEq)]
enum MoveError {IsNotCattyCorner, BlockedByPiece, FromPieceMissing,
                WrongDirection}


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
        println!(       "            ---- Board ----           ");
        let mut board = "     0   1   2   3   4   5   6   7  \n".to_string();
        board = board + "   ---------------------------------\n";
        for i in 0 .. 8 {
            board = board + &format!(" {} |", i);
            for j in 0 .. 8 {
                match self.places[i][j] {
                    Some(Piece {player, .. }) => board = board + &format!(" {} |", player),
                    None => board = board + "   |",
                }
            }
            board = board + "\n   ---------------------------------\n";
        }
        println!("{}", board);
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
                    self.places[i][j] = Some(Piece {player: 2,
                                                    king: false});
                }
            }
            for i in 5 .. 8 {
                if (i + j) % 2 == 0 {
                    self.places[i][j] = Some(Piece {player: 1,
                                                    king: false});
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
    ///     (regardless of who's turn it is)
    ///
    /// This will check that:
    /// `from` is not empty
    /// `to` is empty
    /// if `from_piece` is not a king:
    ///     `to` is in the forward direction for the player of `from`
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
    /// ```
    fn valid_move(&self, from: Position, to: Position) -> Result<(), MoveError> {
        let from_piece: Piece;

        match self.get_at_position(from) {
            Some(piece) => from_piece = piece,
            None => return Err(MoveError::FromPieceMissing)
        }

        match self.get_at_position(to) {
            Some(_) => return Err(MoveError::BlockedByPiece),
            None => ()
        }

        if from_piece.king {
            if (from_piece.player == 1 && from.row < to.row)
                    || (from_piece.player == 2 && from.row > to.row) {
                return Err(MoveError::WrongDirection);
            }
        }

        // Catty-corner check
        if !from.is_catty_corner(to) {
            return Err(MoveError::IsNotCattyCorner);
        }

        // TODO: Handle double-catty-corner

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
        assert_eq!(Ok(()), board.valid_move(Position {row: 2, column: 0},
                                            Position {row: 3, column: 1}));
        assert_eq!(Err(MoveError::BlockedByPiece),
                   board.valid_move(Position {row: 1, column: 1},
                                    Position {row: 2, column: 2}));
        assert_eq!(Err(MoveError::IsNotCattyCorner),
                   board.valid_move(Position {row: 1, column: 1},
                                    Position {row: 3, column: 2}));
        assert_eq!(Err(MoveError::FromPieceMissing),
                   board.valid_move(Position {row: 2, column: 1},
                                    Position {row: 3, column: 1}));
        board.move_piece(Position {row: 2, column: 0},
                         Position {row: 3, column: 1});
        board.move_piece(Position {row: 3, column: 1},
                         Position {row: 4, column: 2});
        board.move_piece(Position {row: 2, column: 2},
                         Position {row: 3, column: 3});

        // Double catty-corner over piece
        assert_eq!(Ok(()), board.valid_move(Position {row: 5, column: 3},
                                            Position {row: 3, column: 1}));
        assert_eq!(Err(MoveError::IsNotCattyCorner),
                   board.valid_move(Position {row: 5, column: 5},
                                    Position {row: 3, column: 7}));
        assert_eq!(Err(MoveError::BlockedByPiece),
                   board.valid_move(Position {row: 5, column: 1},
                                    Position {row: 3, column: 3}));
    }
}
