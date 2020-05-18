use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

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

    draw_board(&mut stdout)?;

    for key_result in stdin.keys() {
        match key_result {
            Ok(key) => match key {
                Key::Esc => break,
                _ => {}
            },
            Err(err) => return Err(err.into()),
        }

        draw_board(&mut stdout)?;

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

fn draw_board(stdout: &mut impl Write) -> Result<()> {
    for dx in 0..(BOARD_WIDTH + 2) {
        draw_wall(stdout, dx, 0)?;
        draw_wall(stdout, dx, BOARD_HEIGHT + 1)?;
    }
    for dy in 1..(BOARD_HEIGHT + 1) {
        draw_wall(stdout, 0, dy)?;
        draw_wall(stdout, BOARD_WIDTH + 1, dy)?;
    }

    stdout.flush()?;

    Ok(())
}

fn draw_wall(stdout: &mut impl Write, x: u16, y: u16) -> Result<()> {
    for dy in 0..3 {
        write!(stdout, "{}██████", termion::cursor::Goto(3 + 6 * x, 2 + 3 * y + dy))?;
    }
    Ok(())
}
