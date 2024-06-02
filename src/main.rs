struct QueenBoard {
    board: Vec<Vec<i32>>,
}

impl QueenBoard {
    fn new(size: usize) -> Self {
        // Initialize the board with 0s
        QueenBoard {
            board: vec![vec![0; size]; size],
        }
    }

    fn print_board(&self) {
        for row in &self.board {
            for &cell in row {
                if cell == -1 {
                    print!("Q ");
                } else {
                    print!("_ ");
                }
            }
            println!(); // Newline after each row
        }
    }

    fn add_queen(&mut self, r: usize, c: usize) -> bool {
        if self.board[r][c] > 0 {
            return false;
        }

        self.modify_board(r, c, 1);
        true
    }

    fn remove_queen(&mut self, r: usize, c: usize) {
        if self.board[r][c] == 1 {
            return;
        }

        self.modify_board(r, c, -1);
    }

    fn modify_board(&mut self, r: usize, c: usize, inc: i32) {
        self.board[r][c] += -inc;

        let mut k = 1;
        while r + k < self.board.len() {
            self.board[r + k][c] += inc;
            if c >= k {
                self.board[r + k][c - k] += inc;
            }
            if c + k < self.board[0].len() {
                self.board[r + k][c + k] += inc;
            }
            k += 1;
        }
    }

    fn solve(&mut self, r: usize) -> bool {
        if r == self.board.len() {
            return true;
        }

        for c in 0..self.board[0].len() {
            if self.add_queen(r, c) {
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
