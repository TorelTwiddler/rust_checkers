fn main() {
    let mut board: Board = Default::default();
    board.print();
    board.new_game();
    board.print();
    println!("\n{:?}", board.move_piece(Position {x: 0, y: 0}, Position {x: 0, y: 1}).is_ok());
    board.print();
}

/// An 8x8 board that is filled with `Option<Piece>`s.
///
/// # Example
/// 
/// ```
/// let mut board: Board = Default::default();
/// ```
struct Board {
    places: [[Option<Piece>; 8]; 8],
}

impl Board {

    /// Prints the board in a human-readable format.
    fn print(&self) {
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
    fn new_game(&mut self) {
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
    fn move_piece(&mut self, from: Position, to: Position) -> Result<(), &str> {
        let piece: Option<Piece> = self.get_at_position(from);
        if piece.is_some() {
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

/// A Piece on the board.
#[derive(Debug, Copy, Clone)]
struct Piece {
    player: i32,
}

/// A position on the board, x being the row, y the column.
#[derive(Debug, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}
