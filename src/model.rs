use std::cmp::min;

pub enum BlockNumber {
    ONE,
}

impl BlockNumber {
    pub fn to_u32(&self) -> u32 {
        match self {
            Self::ONE => 1,
        }
    }
}

pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

pub struct Block {
    pub number: BlockNumber,
    pub position: Position,
}

impl Block {
    pub fn new(number: BlockNumber, x: u32, y: u32) -> Self {
        Self {
            number,
            position: Position::new(x, y),
        }
    }
}

pub enum Direction {
    LEFT,
    RIGHT,
    DOWN,
}

pub struct Board {
    pub width: u32,
    pub height: u32,
    pub current_block: Block,
    pub blocks: Vec<Block>,
}

impl Board {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            current_block: Block::new(BlockNumber::ONE, (width - 1) / 2, 0),
            blocks: vec![],
        }
    }

    pub fn move_current_block(&mut self, direction: Direction) {
        let prev_x = self.current_block.position.x;
        let prev_y = self.current_block.position.y;
        match direction {
            Direction::LEFT => {
                self.current_block.position.x = if prev_x > 0 { prev_x - 1 } else { 0 }
            }
            Direction::RIGHT => self.current_block.position.x = min(self.width - 1, prev_x + 1),
            Direction::DOWN => self.current_block.position.y = min(self.height - 1, prev_y + 1),
        };
    }
}
