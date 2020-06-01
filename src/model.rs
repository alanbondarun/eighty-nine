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

pub struct Board {
    pub current_block: Block,
    pub blocks: Vec<Block>,
}

impl Board {
    pub fn new(width: u32, _height: u32) -> Self {
        Self {
            current_block: Block::new(BlockNumber::ONE, (width - 1) / 2, 0),
            blocks: vec![],
        }
    }
}
