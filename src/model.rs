use rand::{thread_rng, Rng};
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq)]
pub enum BlockNumber {
    One,
    Two,
    Three,
    Five,
    Eight,
    Thirteen,
    TwentyOne,
    ThrityFour,
    FiftyFive,
    EightyNine,
}

impl BlockNumber {
    pub fn to_u32(self) -> u32 {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Five => 5,
            Self::Eight => 8,
            Self::Thirteen => 13,
            Self::TwentyOne => 21,
            Self::ThrityFour => 34,
            Self::FiftyFive => 55,
            Self::EightyNine => 89,
        }
    }

    pub fn next(self) -> Option<Self> {
        match self {
            Self::One => Some(Self::Two),
            Self::Two => Some(Self::Three),
            Self::Three => Some(Self::Five),
            Self::Five => Some(Self::Eight),
            Self::Eight => Some(Self::Thirteen),
            Self::Thirteen => Some(Self::TwentyOne),
            Self::TwentyOne => Some(Self::ThrityFour),
            Self::ThrityFour => Some(Self::FiftyFive),
            Self::FiftyFive => Some(Self::EightyNine),
            Self::EightyNine => None,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Copy, Clone, PartialEq)]
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

pub struct MergeableBlocks {
    pub from: Block,
    pub to: Block,
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
        let random_value = thread_rng().gen_range(0, 2);
        Block::new(
            if random_value < 1 {
                BlockNumber::One
            } else {
                BlockNumber::Two
            },
            (width - 1) / 2,
            0,
        )
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

    pub fn update(&mut self) {
        loop {
            let mergeable_blocks = self.mergeable_blocks();
            if mergeable_blocks.is_empty() {
                break;
            }
            for blocks in mergeable_blocks {
                self.merge_blocks(blocks);
            }
        }
    }

    pub fn mergeable_blocks(&self) -> Vec<MergeableBlocks> {
        let blocks_by_position = self.blocks_by_position();
        let mut mergeable_blocks = vec![];

        for x in 0..self.width - 1 {
            for y in 0..self.height {
                if let Some(left_block) = blocks_by_position.get(&(x, y)) {
                    if let Some(right_block) = blocks_by_position.get(&(x + 1, y)) {
                        if left_block
                            .number
                            .next()
                            .map_or(false, |n| n == right_block.number)
                            && right_block.number.next().is_some()
                        {
                            mergeable_blocks.push(MergeableBlocks {
                                from: **left_block,
                                to: **right_block,
                            })
                        }
                        if right_block
                            .number
                            .next()
                            .map_or(false, |n| n == left_block.number)
                            && left_block.number.next().is_some()
                        {
                            mergeable_blocks.push(MergeableBlocks {
                                from: **right_block,
                                to: **left_block,
                            })
                        }
                    }
                }
            }
        }

        for x in 0..self.width {
            for y in 0..self.height - 1 {
                if let Some(bottom_block) = blocks_by_position.get(&(x, y)) {
                    if let Some(top_block) = blocks_by_position.get(&(x, y + 1)) {
                        if bottom_block
                            .number
                            .next()
                            .map_or(false, |n| n == top_block.number)
                            && top_block.number.next().is_some()
                        {
                            mergeable_blocks.push(MergeableBlocks {
                                from: **bottom_block,
                                to: **top_block,
                            })
                        }
                        if top_block
                            .number
                            .next()
                            .map_or(false, |n| n == bottom_block.number)
                            && bottom_block.number.next().is_some()
                        {
                            mergeable_blocks.push(MergeableBlocks {
                                from: **top_block,
                                to: **bottom_block,
                            })
                        }
                    }
                }
            }
        }

        mergeable_blocks
    }

    fn blocks_by_position(&self) -> HashMap<(u32, u32), &Block> {
        self.blocks
            .iter()
            .map(|block| ((block.position.x, block.position.y), block))
            .collect()
    }

    pub fn merge_blocks(&mut self, mergeable_blocks: MergeableBlocks) {
        self.blocks
            .retain(|block| block != &mergeable_blocks.from && block != &mergeable_blocks.to);
        self.blocks.push(Block {
            position: mergeable_blocks.to.position,
            number: mergeable_blocks.to.number.next().unwrap(),
        })
    }
}
