extern crate termcolor;
extern crate termion;
mod tictactoe;

use crate::tictactoe::game::{Game, Direction};
use std::process::Command;

use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    Command::new("clear").status().unwrap();

    let mut game = Game::new();
    let mut exit = false;

    loop {
        game.draw();

        // Attiva la modalità raw per intercettare l'input
        let mut stdout = stdout().into_raw_mode().unwrap();

        // Itera sugli eventi di input
        for c in stdin().keys() {
            match c.unwrap() {
                Key::Up => {
                    game.move_cursor(&Direction::Up);
                    Command::new("clear").status().unwrap();
                    game.draw();
                }
                Key::Down => {
                    game.move_cursor(&Direction::Down);
                    Command::new("clear").status().unwrap();
                    game.draw();
                }
                Key::Left => {
                    game.move_cursor(&Direction::Left);
                    Command::new("clear").status().unwrap();
                    game.draw();
                }
                Key::Right => {
                    game.move_cursor(&Direction::Right);
                    Command::new("clear").status().unwrap();
                    game.draw();
                }
                Key::Ctrl('c') => {
                    write!(stdout, "BYE!\r\n").unwrap();
                    exit = true;
                    break;
                }
                Key::Char('\n') => {
                    let current_playe = game.current_player();

                    if game.get_cell(game.current_cell) == game.player1.get_symbol()
                        || game.get_cell(game.current_cell) == game.player2.get_symbol()
                    {
                        print!("This cell is already taken by player ");
                        game.draw_cell(&game.get_cell(game.current_cell), false);
                        println!("\r");
                        continue;
                    } else {
                        game.set_cell(game.current_cell, &current_playe.get_symbol());
                        break;
                    }
                }
                Key::Char(e) => {
                    if let Ok(number) = e.to_string().parse::<usize>() {
                        if number < 1 || number > 9 {
                            println!("The cell must be between 1 and 9.");
                            continue;
                        }

                        game.current_cell = number - 1;
                        Command::new("clear").status().unwrap();
                        game.draw();
                    }
                }
                _ => {}
            }

            stdout.flush().unwrap();
        }

        if exit {
            break;
        }

        if game.check() {
            print!("Player ");
            game.draw_cell(&game.current_player().get_symbol(), false);
            println!(" wins!\r");
            break;
        }

        if game.is_full() {
            println!("Draw!\r");
            break;
        }

        game.switch_player();
        Command::new("clear").status().unwrap();
    }
}
