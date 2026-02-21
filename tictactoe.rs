use std::io::{self, Write};
use rand::Rng;

const BOARD_SIZE: usize = 3;

#[derive(Clone, PartialEq, Debug, Copy)]
enum Cell {
    Empty,
    X,
    O,
}

#[derive(Clone, PartialEq, Debug)]
enum Player {
    Human,
    ComputerEasy,
    ComputerHard,
}

struct Game {
    board: [[Cell; BOARD_SIZE]; BOARD_SIZE],
    current_player: Cell,
}

impl Game {
    fn new() -> Self {
        Game {
            board: [[Cell::Empty; BOARD_SIZE]; BOARD_SIZE],
            current_player: Cell::X,
        }
    }

    fn display_board(&self) {
        println!("-------------");
        for row in &self.board {
            print!("| ");
            for cell in row {
                match cell {
                    Cell::Empty => print!(". "),
                    Cell::X => print!("X "),
                    Cell::O => print!("O "),
                }
            }
            println!("|");
        }
        println!("-------------");
    }

    fn is_winner(&self, player: &Cell) -> bool {
        for i in 0..BOARD_SIZE {
            if self.board[i].iter().all(|c| *c == *player) {
                return true;
            }
            if (0..BOARD_SIZE).all(|j| self.board[j][i] == *player) {
                return true;
            }
        }
        if (0..BOARD_SIZE).all(|i| self.board[i][i] == *player) {
            return true;
        }
        if (0..BOARD_SIZE).all(|i| self.board[i][BOARD_SIZE - 1 - i] == *player) {
            return true;
        }
        false
    }

    fn is_draw(&self) -> bool {
        self.board.iter().all(|row| row.iter().all(|cell| *cell != Cell::Empty))
    }

    fn make_move(&mut self, row: usize, col: usize) -> bool {
        if self.board[row][col] == Cell::Empty {
            self.board[row][col] = self.current_player;
            true
        } else {
            false
        }
    }

    fn switch_player(&mut self) {
        self.current_player = match self.current_player {
            Cell::X => Cell::O,
            Cell::O => Cell::X,
            _ => unreachable!(),
        };
    }

    fn get_random_move(&self) -> Option<(usize, usize)> {
        let mut rng = rand::thread_rng();
        let empty_cells: Vec<(usize, usize)> = (0..BOARD_SIZE)
            .flat_map(|row| {
                (0..BOARD_SIZE).filter_map(move |col| {
                    (self.board[row][col] == Cell::Empty).then_some((row, col))
                })
            })
            .collect();
        if empty_cells.is_empty() {
            return None;
        }
        Some(empty_cells[rng.gen_range(0..empty_cells.len())])
    }

    fn minimax(&mut self, depth: usize, is_maximizing: bool) -> i32 {
        if self.is_winner(&Cell::X) {
            return 10 - depth as i32;
        }
        if self.is_winner(&Cell::O) {
            return depth as i32 - 10;
        }
        if self.is_draw() {
            return 0;
        }

        if is_maximizing {
            let mut best_score = i32::MIN;
            for i in 0..BOARD_SIZE {
                for j in 0..BOARD_SIZE {
                    if self.board[i][j] == Cell::Empty {
                        self.board[i][j] = Cell::X;
                        let score = self.minimax(depth + 1, false);
                        self.board[i][j] = Cell::Empty;
                        best_score = best_score.max(score);
                    }
                }
            }
            best_score
        } else {
            let mut best_score = i32::MAX;
            for i in 0..BOARD_SIZE {
                for j in 0..BOARD_SIZE {
                    if self.board[i][j] == Cell::Empty {
                        self.board[i][j] = Cell::O;
                        let score = self.minimax(depth + 1, true);
                        self.board[i][j] = Cell::Empty;
                        best_score = best_score.min(score);
                    }
                }
            }
            best_score
        }
    }

    fn get_best_move(&mut self) -> (usize, usize) {
        let mut best_score = i32::MIN;
        let mut best_move = (0, 0);

        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if self.board[i][j] == Cell::Empty {
                    self.board[i][j] = Cell::X;
                    let score = self.minimax(0, false);
                    self.board[i][j] = Cell::Empty;
                    if score > best_score {
                        best_score = score;
                        best_move = (i, j);
                    }
                }
            }
        }
        best_move
    }
}

fn main() {
    let mut game = Game::new();
    let player1 = Player::Human;

    println!("Welcome to Tic Tac Toe!");
    println!("Select game mode:");
    println!("1. Player vs Player");
    println!("2. Player vs Computer (Easy)");
    println!("3. Player vs Computer (Hard)");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let player2 = match input.trim() {
        "1" => Player::Human,
        "2" => Player::ComputerEasy,
        "3" => Player::ComputerHard,
        _ => {
            println!("Invalid selection. Exiting...");
            return;
        }
    };

    loop {
        game.display_board();
        println!("Current Player: {:?}", game.current_player);

        let (row, col) = match game.current_player {
            Cell::X => match player1 {
                Player::Human => get_player_move(),
                _ => unreachable!(),
            },
            Cell::O => match player2 {
                Player::Human => get_player_move(),
                Player::ComputerEasy => match game.get_random_move() {
                    Some(random_move) => random_move,
                    None => {
                        game.display_board();
                        println!("It's a draw!");
                        break;
                    }
                },
                Player::ComputerHard => game.get_best_move(),
            },
            Cell::Empty => unreachable!(),
        };

        if game.make_move(row, col) {
            if game.is_winner(&game.current_player) {
                game.display_board();
                println!("Player {:?} wins!", game.current_player);
                break;
            }
            if game.is_draw() {
                game.display_board();
                println!("It's a draw!");
                break;
            }
            game.switch_player();
        } else {
            println!("Invalid move. Try again.");
        }
    }
}

fn get_player_move() -> (usize, usize) {
    loop {
        print!("Enter your move (row and column): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.len() == 2 {
            if let (Ok(row), Ok(col)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                if row < BOARD_SIZE && col < BOARD_SIZE {
                    return (row, col);
                }
            }
        }
        println!("Invalid input. Please enter two numbers separated by a space (e.g., 1 2).");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_random_move_picks_only_empty_cell() {
        let mut game = Game::new();
        game.board = [
            [Cell::X, Cell::O, Cell::X],
            [Cell::O, Cell::X, Cell::O],
            [Cell::X, Cell::Empty, Cell::O],
        ];

        assert_eq!(game.get_random_move(), Some((2, 1)));
    }
}
