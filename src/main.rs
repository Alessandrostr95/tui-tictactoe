extern crate termcolor;
extern crate termion;
mod tictactoe;

use crate::tictactoe::game::{Direction, Game};
use std::process::Command;

use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    // utility function to clear the screen
    let clear = || {
        Command::new("clear").status().unwrap();
    };
    clear();

    // Create a new game
    let mut game = Game::new();
    let mut exit = false;

    loop {
        // draw the game
        game.draw();

        // active raw mode to intercept input
        let mut stdout = stdout().into_raw_mode().unwrap();

        // iterate over all the keys pressed
        for c in stdin().keys() {
            match c.unwrap() {
                Key::Up => {
                    game.move_cursor(&Direction::Up);
                    clear();
                    game.draw();
                }
                Key::Down => {
                    game.move_cursor(&Direction::Down);
                    clear();
                    game.draw();
                }
                Key::Left => {
                    game.move_cursor(&Direction::Left);
                    clear();
                    game.draw();
                }
                Key::Right => {
                    game.move_cursor(&Direction::Right);
                    clear();
                    game.draw();
                }
                // if the user press ctrl+c, exit the game
                Key::Ctrl('c') => {
                    write!(stdout, "BYE!\r\n").unwrap();
                    exit = true;
                    break;
                }
                // if the user press enter, set the cell
                Key::Char('\n') => {
                    // get the current player
                    let current_playe = game.current_player();

                    // check if the cell is already taken
                    if game.get_cell(game.current_cell) == game.player1.get_symbol()
                        || game.get_cell(game.current_cell) == game.player2.get_symbol()
                    {
                        // if it is, print an error message
                        print!("This cell is already taken by player ");
                        game.draw_cell(&game.get_cell(game.current_cell), false);
                        println!("\r");
                        // and continue the loop
                        continue;
                    } else {
                        // if it is not, set the cell
                        game.set_cell(game.current_cell, &current_playe.get_symbol());
                        // and break the loop
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
                        clear();
                        game.draw();
                    }
                }
                _ => {}
            }

            stdout.flush().unwrap();
        }

        // if the user pressed ctrl+c, exit the game
        if exit {
            break;
        }

        // check if the game is over
        if game.check() {
            clear();
            game.draw();

            print!("Player ");
            game.draw_cell(&game.current_player().get_symbol(), false);
            println!(" wins!\r");
            break;
        }

        // check if the game is a draw
        if game.is_full() {
            clear();
            game.draw();

            println!("Draw!\r");
            break;
        }

        // switch the player
        game.switch_player();
        // clear the screen
        clear();
    }
}
