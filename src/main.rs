use crate::model::{Block, Board, Direction};
use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod model;

const BOARD_WIDTH: u16 = 6;
const BOARD_HEIGHT: u16 = 12;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode()?;

    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    )?;
    stdout.flush()?;

    let mut board = Board::new(BOARD_WIDTH as u32, BOARD_HEIGHT as u32);
    draw_board(&mut stdout, &board)?;

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

        draw_board(&mut stdout, &board)?;

        sleep(Duration::from_micros(1_000_000 / 60))
    }

    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Show
    )?;
    stdout.flush()?;

    Ok(())
}

fn draw_board(stdout: &mut impl Write, board: &Board) -> Result<()> {
    write!(stdout, "{}", termion::clear::All)?;
    for dx in 0..(BOARD_WIDTH + 2) {
        draw_wall(stdout, dx, 0)?;
        draw_wall(stdout, dx, BOARD_HEIGHT + 1)?;
    }
    for dy in 1..(BOARD_HEIGHT + 1) {
        draw_wall(stdout, 0, dy)?;
        draw_wall(stdout, BOARD_WIDTH + 1, dy)?;
    }
    draw_block(stdout, &board.current_block)?;
    for block in &board.blocks {
        draw_block(stdout, block)?;
    }

    stdout.flush()?;

    Ok(())
}

fn draw_wall(stdout: &mut impl Write, x: u16, y: u16) -> Result<()> {
    for dy in 0..3 {
        write!(
            stdout,
            "{}██████",
            termion::cursor::Goto(3 + 6 * x, 2 + 3 * y + dy)
        )?;
    }
    Ok(())
}

fn draw_block(stdout: &mut impl Write, block: &Block) -> Result<()> {
    write!(
        stdout,
        "{}┌────┐",
        termion::cursor::Goto(
            (9 + 6 * block.position.x) as u16,
            (5 + 3 * block.position.y) as u16
        )
    )?;
    write!(
        stdout,
        "{}│ {: >2} │",
        termion::cursor::Goto(
            (9 + 6 * block.position.x) as u16,
            (6 + 3 * block.position.y) as u16
        ),
        block.number.to_u32()
    )?;
    write!(
        stdout,
        "{}└────┘",
        termion::cursor::Goto(
            (9 + 6 * block.position.x) as u16,
            (7 + 3 * block.position.y) as u16
        )
    )?;
    Ok(())
}
