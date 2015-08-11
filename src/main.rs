mod board;
mod position;
mod piece;

use board::Board;
use position::Position;

fn main() {
    let mut board: Board = Default::default();
    board.print();
    board.new_game();
    board.print();
    println!("\n{:?}", board.move_piece(Position {row: 2, column: 0}, Position {row: 3, column: 1}));
    board.print();
}
