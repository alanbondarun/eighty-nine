use crate::model::{Block, Board, Direction};
use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod model;
mod render;

const BOARD_WIDTH: u16 = 6;
const BOARD_HEIGHT: u16 = 12;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode()?;

    render::clear(&mut stdout)?;
    stdout.flush()?;

    let mut board = Board::new(BOARD_WIDTH as u32, BOARD_HEIGHT as u32);
    render::draw_board(&mut stdout, &board)?;
    stdout.flush()?;

    for key_result in stdin.keys() {
        match key_result {
            Ok(key) => match key {
                Key::Esc => break,
                Key::Left => board.move_current_block(Direction::LEFT),
                Key::Right => board.move_current_block(Direction::RIGHT),
                Key::Down => board.move_current_block(Direction::DOWN),
                _ => {}
            },
            Err(err) => return Err(err.into()),
        }

        render::draw_board(&mut stdout, &board)?;
        stdout.flush()?;

        sleep(Duration::from_micros(1_000_000 / 60))
    }

    render::cleanup(&mut stdout)?;
    stdout.flush()?;

    Ok(())
}
