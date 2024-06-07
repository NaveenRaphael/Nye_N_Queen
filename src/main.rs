use std::ops::{AddAssign, Index, IndexMut};

use num_traits::{NumAssign, Unsigned};

///
///A trait for a generic unsized int number
trait MyNum: NumAssign + Unsigned + Clone + Copy {}

///An enum for the direction of change; can increase or decrease.
/// This used to be a bool earlier!
/// (I did not want to make the change into a signed integer because unsigned + signed became complicated)
#[derive(Clone, Copy)]
enum Change {
    Increase,
    Decrease,
}

///The state of a generic square on the board.
/// It either has a queen, or that square is viewed by k Queens, where k is an unsigned int (can be 0)
#[derive(Clone, Copy)]
enum CellState<T: MyNum> {
    Queen,
    QueenSeen(T),
}

impl<T: MyNum> CellState<T> {
    ///Checks if this cell can have a queen placed
    fn queen_candidate(&self) -> bool {
        match self {
            CellState::QueenSeen(i) if i.is_zero() => true,
            _ => false,
        }
    }
}

impl<T: MyNum> Default for CellState<T> {
    fn default() -> Self {
        CellState::QueenSeen(T::zero())
    }
}

//Because heh
impl<T: MyNum> AddAssign<Change> for CellState<T> {
    fn add_assign(&mut self, rhs: Change) {
        match (self, rhs) {
            (CellState::Queen, _) => unreachable!(),
            (CellState::QueenSeen(i), Change::Increase) => *i += T::one(),
            (CellState::QueenSeen(i), Change::Decrease) => *i -= T::one(),
        }
    }
}

///
/// Changed this to an array of array; as the number of cells dont change
struct QueenBoard<T: MyNum, const N: usize> {
    board: [[CellState<T>; N]; N],
}

/// This is an overkill
impl<T: MyNum, const N: usize> Index<(usize, usize)> for QueenBoard<T, N> {
    type Output = CellState<T>;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.board[index.0 as usize][index.1 as usize]
    }
}

impl<T: MyNum, const N: usize> IndexMut<(usize, usize)> for QueenBoard<T, N> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.board[index.0 as usize][index.1 as usize]
    }
}

impl<T: MyNum, const N: usize> QueenBoard<T, N> {
    fn new() -> Self {
        // Initialize the board with 0s
        QueenBoard {
            board: [[CellState::default(); N]; N],
        }
    }

    fn print_board(&self) {
        for row in &self.board {
            for cell in row {
                match cell {
                    CellState::Queen => print!("Q "),
                    CellState::QueenSeen(_) => print!("_ "),
                };
            }
            println!(); // Newline after each row
        }
    }
    fn can_place_queen(&self, r: usize, c: usize) -> bool {
        self[(r, c)].queen_candidate()
    }

    fn add_queen(&mut self, r: usize, c: usize) {
        self[(r, c)] = CellState::Queen;
        self.modify_board_other_cells(r, c, Change::Increase);
    }

    fn remove_queen(&mut self, r: usize, c: usize) {
        self[(r, c)] = CellState::default();
        self.modify_board_other_cells(r, c, Change::Decrease);
    }

    fn modify_board_other_cells(&mut self, r: usize, c: usize, inc: Change) {
        for k in 1..N {
            if r + k < N {
                self[(r + k, c)] += inc;
            }
            if r + k < N && c + k < N {
                self[(r + k, c + k)] += inc;
            }
            if c + k < N {
                self[(r, c + k)] += inc;
            }
            if r >= k && c + k < N {
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
            if c >= k && r + k < N {
                self[(r + k, c - k)] += inc;
            }
        }
    }

    fn solve(&mut self, r: usize) -> bool {
        if r == N {
            return true;
        }

        for c in 0..N {
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

impl MyNum for u16 {}

fn main() {
    let mut board: QueenBoard<u16, 8> = QueenBoard::new(); // Create an 8x8 board

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
