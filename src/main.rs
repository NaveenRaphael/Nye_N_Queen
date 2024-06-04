use std::ops::{AddAssign, Index, IndexMut};

#[derive(Clone, Copy)]
enum CellState {
    Queen,
    QueenSeen(u8),
}

impl CellState {
    fn queen_candidate(&self) -> bool {
        matches!(self, CellState::QueenSeen(0))
    }
}
impl Default for CellState {
    fn default() -> Self {
        CellState::QueenSeen(0)
    }
}
impl AddAssign<i8> for CellState {
    fn add_assign(&mut self, rhs: i8) {
        match (self, rhs > 0) {
            (CellState::Queen, _) => unreachable!(),
            (CellState::QueenSeen(i), true) => *i += rhs as u8,
            (CellState::QueenSeen(i), false) => *i -= (-rhs) as u8,
        }
    }
}

struct QueenBoard {
    len: u8,
    board: Vec<Vec<CellState>>,
}

impl Index<(u8, u8)> for QueenBoard {
    type Output = CellState;

    fn index(&self, index: (u8, u8)) -> &Self::Output {
        &self.board[index.0 as usize][index.1 as usize]
    }
}

impl IndexMut<(u8, u8)> for QueenBoard {
    fn index_mut(&mut self, index: (u8, u8)) -> &mut Self::Output {
        &mut self.board[index.0 as usize][index.1 as usize]
    }
}

impl QueenBoard {
    fn new(size: u8) -> Self {
        // Initialize the board with 0s
        QueenBoard {
            len: size as u8,
            board: vec![vec![CellState::default(); size as usize]; size as usize],
        }
    }

    fn print_board(&self) {
        for row in &self.board {
            for &cell in row {
                match cell {
                    CellState::Queen => print!("Q "),
                    CellState::QueenSeen(_) => print!("_ "),
                };
            }
            println!(); // Newline after each row
        }
    }
    fn can_place_queen(&self, r: u8, c: u8) -> bool {
        self[(r, c)].queen_candidate()
    }

    fn add_queen(&mut self, r: u8, c: u8) {
        self[(r, c)] = CellState::Queen;
        self.modify_board_other_cells(r, c, true);
    }

    fn remove_queen(&mut self, r: u8, c: u8) {
        self[(r, c)] = CellState::default();
        self.modify_board_other_cells(r, c, false);
    }

    fn modify_board_other_cells(&mut self, r: u8, c: u8, to_increase: bool) {
        let inc = if to_increase { 1 } else { -1 };
        for k in 1..8 {
            if r + k < self.len {
                self[(r + k, c)] += inc;
            }
            if r + k < self.len && c + k < self.len {
                self[(r + k, c + k)] += inc;
            }
            if c + k < self.len {
                self[(r, c + k)] += inc;
            }
            if r >= k && c + k < self.len {
                self[(r - k, c + k)] += inc;
            }
            if r >= k {
                self[(r - k, c)] += inc;
            }
            if r >= k && c >= k {
                self[(r - k, c - k)] += inc;
            }
            if c >= k {
                self[(r, c - k)] += inc;
            }
            if c >= k && r + k < self.len {
                self[(r + k, c - k)] += inc;
            }
        }
    }

    fn solve(&mut self, r: u8) -> bool {
        if r == self.len {
            return true;
        }

        for c in 0..self.len {
            if self.can_place_queen(r, c) {
                self.add_queen(r, c);
                if self.solve(r + 1) {
                    return true;
                }
                self.remove_queen(r, c);
            }
        }
        false
    }
}

fn main() {
    let mut board = QueenBoard::new(8); // Create an 8x8 board

    board.add_queen(0, 0); // Add a queen to (0, 0)
    board.add_queen(1, 2); // Add a queen to (1, 2)
    board.print_board(); // Print the board
    println!();

    board.remove_queen(0, 0); // Remove the queen at (0, 0)
    board.remove_queen(1, 2); // Remove the queen at (1, 2)
    board.print_board(); // Print the board
    println!();

    board.solve(0); // Solve the N-Queens problem
    board.print_board(); // Print the board
}
