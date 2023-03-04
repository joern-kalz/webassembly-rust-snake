mod vec2d;

use std::collections::VecDeque;

use self::vec2d::Vec2D;

pub const SCREEN_WIDTH: usize = 30;
pub const SCREEN_HEIGHT: usize = 30;
const BYTES_PER_PIXEL: usize = 4;
const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT * BYTES_PER_PIXEL;

pub struct World {
    pub screen: [u8; SCREEN_SIZE],
    direction: Vec2D,
    snake: VecDeque<Vec2D>,
    food: Vec2D,
}

impl World {
    pub fn new() -> World {
        let mut screen = [0u8; SCREEN_SIZE];

        let mut i = 3;
        while i < SCREEN_SIZE {
            screen[i] = 255;
            i += 4;
        }

        World {
            screen,
            direction: Vec2D { x: 1, y: 0 },
            snake: VecDeque::new(),
            food: Vec2D { x: 5, y: 5 },
        }
    }

    pub fn tick(&mut self) {}

    pub fn click(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let world = World::new();
        assert_eq!(world.snake.len(), 0);
    }
}
