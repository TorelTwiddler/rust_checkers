fn main() {
    let mut board: Board = Default::default();
    board.print();
    board.new_game();
    board.print();
}

struct Board {
    places: [[Option<Piece>; 8]; 8],
}

impl Board {
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
}

impl Default for Board {
    fn default() -> Board {
        Board {
            places: [[None; 8]; 8],
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Piece {
    player: i32,
}
