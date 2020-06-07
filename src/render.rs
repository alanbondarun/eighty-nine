use crate::model::{Block, Board};
use crate::Result;
use std::io::Write;

pub fn clear(stdout: &mut impl Write) -> Result<()> {
    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    )
    .map_err(|err| err.into())
}

pub fn cleanup(stdout: &mut impl Write) -> Result<()> {
    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Show
    )
    .map_err(|err| err.into())
}

pub fn draw_board(stdout: &mut impl Write, board: &Board) -> Result<()> {
    write!(stdout, "{}", termion::clear::All)?;
    for dx in 0..(board.width + 2) {
        draw_wall(stdout, dx as u16, 0)?;
        draw_wall(stdout, dx as u16, board.height as u16 + 1)?;
    }
    for dy in 1..(board.height + 1) {
        draw_wall(stdout, 0, dy as u16)?;
        draw_wall(stdout, board.width as u16 + 1, dy as u16)?;
    }
    draw_block(stdout, &board.current_block)?;
    for block in &board.blocks {
        draw_block(stdout, block)?;
    }

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
