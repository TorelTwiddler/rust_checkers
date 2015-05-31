
const BOARD_SIZE : usize = 8;

fn main() {
  let mut board: Board = Default::default();
  board.print();
  board.places[2][3] = Some(Piece {id: 4});
  board.print();
}

struct Board {
    places: [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE],
}

impl Board { fn print(&self) {
        println!("---- Board ----");
        for i in 1 .. BOARD_SIZE {
            for j in 1 .. BOARD_SIZE {
                match self.places[i][j] {
                    Some(Piece {id, .. }) => print!("{:?}", id),
                    None => print!("0"),
                }
            }
            println!(" ");
        }
    }
}

impl Default for Board {
    fn default() -> Board {
        Board {
            places: [[None; BOARD_SIZE]; BOARD_SIZE],
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Piece {
    id: i32,
}
