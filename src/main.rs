
const BOARD_SIZE: i32 = 8;

fn main() {
  let mut board: Board = Default::default();
  board.print();
  board.places[2][3] = 4;
  board.print();
}

struct Board {
    places: [[i32; 8]; 8],
}

impl Board {
    fn print(&self) {
        println!("---- Board ----");
        for i in 1 .. 8 {
            for j in 1 .. 8 {
                print!("{}", self.places[i][j]);
            }
            println!(" ");
        }
    }
}

impl Default for Board {
    fn default() -> Board {
        Board {
            places: [[0; 8]; 8],
        }
    }
}

struct Piece {
    id: i32,
    name: str,
}

// impl Display for Piece {
//     fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
//         formatter.write_str(self.id)
//     }
// }
