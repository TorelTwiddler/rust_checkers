/// A Piece on the board.
#[derive(Debug, Copy, Clone)]
pub struct Piece {
    pub player: i32,
    pub king: bool,
}
