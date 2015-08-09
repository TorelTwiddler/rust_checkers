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
    println!("\n{:?}", board.move_piece(Position {x: 0, y: 2}, Position {x: 1, y: 3}).is_ok());
    board.print();
}
