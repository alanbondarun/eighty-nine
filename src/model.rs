#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Copy, Clone)]
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
            current_block: Board::default_current_block(width),
            blocks: vec![],
        }
    }

    fn default_current_block(width: u32) -> Block {
        Block::new(BlockNumber::ONE, (width - 1) / 2, 0)
    }

    pub fn move_current_block(&mut self, direction: Direction) {
        let prev_x = self.current_block.position.x;
        let prev_y = self.current_block.position.y;

        match direction {
            Direction::LEFT => {
                self.current_block.position.x = if self.movable_to_left() {
                    prev_x - 1
                } else {
                    prev_x
                };
            }
            Direction::RIGHT => {
                self.current_block.position.x = if self.movable_to_right() {
                    prev_x + 1
                } else {
                    prev_x
                };
            }
            Direction::DOWN => {
                if self.movable_to_down() {
                    self.current_block.position.y = prev_y + 1;
                } else {
                    self.blocks.push(self.current_block);
                    self.current_block = Board::default_current_block(self.width);
                }
            }
        };
    }

    fn movable_to_down(&self) -> bool {
        let prev_x = self.current_block.position.x;
        let prev_y = self.current_block.position.y;

        let highest_block = self
            .blocks
            .iter()
            .filter(|block| block.position.x == prev_x)
            .min_by_key(|block| block.position.y);
        match highest_block {
            Some(block) => {
                prev_y
                    < if block.position.y > 0 {
                        block.position.y - 1
                    } else {
                        0
                    }
            }
            None => prev_y < self.height - 1,
        }
    }

    fn movable_to_left(&self) -> bool {
        let prev_x = self.current_block.position.x;
        let prev_y = self.current_block.position.y;

        let is_blocking = self
            .blocks
            .iter()
            .any(|block| block.position.y == prev_y && block.position.x + 1 == prev_x);

        (!is_blocking) && prev_x > 0
    }

    fn movable_to_right(&self) -> bool {
        let prev_x = self.current_block.position.x;
        let prev_y = self.current_block.position.y;

        let is_blocking = self
            .blocks
            .iter()
            .any(|block| block.position.y == prev_y && block.position.x == prev_x + 1);

        (!is_blocking) && prev_x < self.width - 1
    }
}
