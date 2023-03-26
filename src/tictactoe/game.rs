extern crate termcolor;
extern crate termion;

use core::cmp::PartialEq;
use std::io::Write;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Player {
    A(char),
    B(char),
}

impl Player {
    /// Get the symbol of the player.
    pub fn get_symbol(&self) -> char {
        match self {
            Player::A(symbol) => *symbol,
            Player::B(symbol) => *symbol,
        }
    }
}

pub struct Game {
    board: Vec<char>,
    pub player1: Player,
    pub player2: Player,
    pub current_player: bool,
    pub current_cell: usize,
}

impl Game {
    /// Create a new game.
    pub fn new() -> Game {
        let board: Vec<char> = (1..10)
            .map(|x| x.to_string().parse::<char>().unwrap())
            .collect();

        let p1 = Player::A('X');
        let p2 = Player::B('O');

        Game {
            board,
            player1: p1,
            player2: p2,
            current_player: true,
            current_cell: 0,
        }
    }

    /// Print the current state of the game.
    pub fn draw(self: &Game) {
        // let symbols = vec!['╬', '═', '╠', '╦', '╩', '╔', '╚', '╝', '╗', '║', '╣'];

        print!("Player ");
        self.draw_cell(&self.current_player().get_symbol(), false);
        println!("\r");

        println!("\n");

        for i in 0..3 {
            let cell = i * 3;

            if i == 0 {
                println!("╔═══╦═══╦═══╗\r");
            } else {
                println!("╠═══╬═══╬═══╣\r")
            }
            print!("║ ");
            self.draw_cell(&self.get_cell(cell), cell == self.current_cell);
            print!(" ║ ");
            self.draw_cell(&self.get_cell(cell + 1), cell + 1 == self.current_cell);
            print!(" ║ ");
            self.draw_cell(&self.get_cell(cell + 2), cell + 2 == self.current_cell);
            println!(" ║\r");
        }
        println!("╚═══╩═══╩═══╝\r");
    }

    /// Print the given symbol in the current color.
    ///
    /// If the `highlights` parameter is true, the cell will be highlighted in yellow.
    ///
    /// If the symbol is the player's symbol, the cell will be colored in blue or red (depending on the player).
    pub fn draw_cell(&self, symb: &char, highlights: bool) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);

        let mut color = Some(Color::Rgb(128, 128, 128));
        let bg_color = Some(Color::Yellow);

        if symb == &self.player1.get_symbol() {
            color = Some(Color::Blue);
        } else if symb == &self.player2.get_symbol() {
            color = Some(Color::Red);
        }

        let mut color_spec = ColorSpec::new();
        color_spec.set_fg(color);
        if highlights {
            color_spec.set_bg(bg_color).set_bold(true);
        }

        stdout.set_color(&color_spec).unwrap();

        write!(&mut stdout, "{}", symb).unwrap();
        stdout.reset().unwrap();
    }

    /// Get the cell at the given index.
    pub fn get_cell(&self, index: usize) -> char {
        self.board[index]
    }

    /// Set the cell at the given index to the given symbol.
    pub fn set_cell(&mut self, index: usize, symb: &char) {
        self.board[index] = *symb;
    }

    /// Switch the current player.
    pub fn switch_player(&mut self) {
        self.current_player = !self.current_player;
    }

    /// Get the current player.
    pub fn current_player(&self) -> &Player {
        if self.current_player {
            &self.player1
        } else {
            &self.player2
        }
    }

    /// Move the cursor in the given direction, if possible.
    pub fn move_cursor(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => {
                if self.current_cell > 2 {
                    self.current_cell -= 3;
                }
                return;
            }
            Direction::Down => {
                if self.current_cell < 6 {
                    self.current_cell += 3;
                }
                return;
            }
            Direction::Left => {
                if self.current_cell % 3 != 0 {
                    self.current_cell -= 1;
                }
                return;
            }
            Direction::Right => {
                if self.current_cell % 3 != 2 {
                    self.current_cell += 1;
                }
                return;
            }
        }
    }

    /// Check if the current player has won.
    pub fn check(&self) -> bool {
        // Get the symbol of the current player.
        let symb = self.current_player().get_symbol();

        for i in 0..3 {
            let first_cell = i * 3;

            // Check if the current player has won horizontally
            if self.get_cell(first_cell) == symb
                && self.get_cell(first_cell + 1) == symb
                && self.get_cell(first_cell + 2) == symb
            {
                return true;
            }
            
            // Check if the current player has won vertically
            if self.get_cell(i) == symb
                && self.get_cell(i + 3) == symb
                && self.get_cell(i + 6) == symb
            {
                return true;
            }
        }

        // Check if the current player has won diagonally (top-left to bottom-right)
        if self.get_cell(0) == symb && self.get_cell(4) == symb && self.get_cell(8) == symb {
            return true;
        }

        // Check if the current player has won diagonally (top-right to bottom-left)
        if self.get_cell(2) == symb && self.get_cell(4) == symb && self.get_cell(6) == symb {
            return true;
        }

        false
    }

    /// Check if the board is full.
    pub fn is_full(&self) -> bool {
        self.board
            .iter()
            .all(|x| x == &self.player1.get_symbol() || x == &self.player2.get_symbol())
    }
}
