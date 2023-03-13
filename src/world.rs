mod vec2d;

use self::vec2d::Vec2D;
use rand::Rng;
use std::collections::VecDeque;

pub const SCREEN_WIDTH: usize = 30;
pub const SCREEN_HEIGHT: usize = 30;
const SCREEN_SIZE_IN_PIXELS: usize = SCREEN_WIDTH * SCREEN_HEIGHT;
const BYTES_PER_PIXEL: usize = 4;
const SCREEN_SIZE_IN_BYTES: usize = SCREEN_SIZE_IN_PIXELS * BYTES_PER_PIXEL;

type Color = [u8; 3];
pub const CLEAR_COLOR: Color = [0, 0, 0];
pub const SNAKE_COLOR: Color = [0, 255, 0];
pub const FOOD_COLOR: Color = [0, 0, 255];
pub const FAIL_COLOR: Color = [255, 0, 0];

const START_LEN: i32 = 7;
const START_Y: i32 = SCREEN_HEIGHT as i32 / 2;
const INVARIANT: &str = "Snake length > 0";

pub struct World {
    pub screen: [u8; SCREEN_SIZE_IN_BYTES],
    direction: Vec2D,
    snake: VecDeque<Vec2D>,
    alive: bool,
}

impl World {
    pub fn new() -> World {
        let mut world = World {
            screen: [255u8; SCREEN_SIZE_IN_BYTES],
            direction: Vec2D::new(1, 0),
            snake: VecDeque::new(),
            alive: true,
        };

        world.clear_screen();
        world.create_initial_snake();
        world.create_initial_food();
        world
    }

    pub fn width() -> u32 {
        SCREEN_WIDTH as u32
    }

    pub fn height() -> u32 {
        SCREEN_HEIGHT as u32
    }

    pub fn tick(&mut self) {
        if self.alive {
            let new_head = self.get_new_head();
            let new_head_pixel = self.get_pixel_at_vec(&new_head);

            self.extend_head_to(&new_head);

            match new_head_pixel {
                FOOD_COLOR => self.create_food(),
                SNAKE_COLOR => self.die(),
                _ => self.shorten_tail(),
            }
        }
    }

    pub fn click(&mut self, x: i32, y: i32) {
        if self.alive {
            let head = self.snake.back().expect(INVARIANT);

            self.direction = match self.direction.x {
                0 => Vec2D::new(if x < head.x { -1 } else { 1 }, 0),
                _ => Vec2D::new(0, if y < head.y { -1 } else { 1 }),
            };

        } else {
            self.direction = Vec2D::new(1, 0);
            self.snake = VecDeque::new();
            self.alive = true;
            self.clear_screen();
            self.create_initial_snake();
            self.create_initial_food();
        }
    }

    fn clear_screen(&mut self) {
        for x in 0..SCREEN_WIDTH as i32 {
            for y in 0..SCREEN_HEIGHT as i32 {
                self.set_pixel(x, y, CLEAR_COLOR);
            }
        }
    }

    fn create_initial_snake(&mut self) {
        for x in 0..START_LEN {
            self.set_pixel(x, START_Y, SNAKE_COLOR);
            self.snake.push_back(Vec2D::new(x, START_Y));
        }
    }

    fn create_initial_food(&mut self) {
        self.set_pixel(START_LEN, START_Y - 2, FOOD_COLOR);
    }

    fn get_new_head(&self) -> Vec2D {
        let moved_head = *self.snake.back().expect(INVARIANT) + self.direction;
        Vec2D::new(
            (moved_head.x + SCREEN_WIDTH as i32) % SCREEN_WIDTH as i32,
            (moved_head.y + SCREEN_HEIGHT as i32) % SCREEN_HEIGHT as i32,
        )
    }

    fn extend_head_to(&mut self, new_head: &Vec2D) {
        self.set_pixel_at_vec(new_head, SNAKE_COLOR);
        self.snake.push_back(*new_head);
    }

    fn shorten_tail(&mut self) {
        let tail = self.snake.pop_front().expect(INVARIANT);
        self.set_pixel_at_vec(&tail, CLEAR_COLOR);
    }

    fn create_food(&mut self) {
        let start_index = rand::thread_rng().gen_range(0..SCREEN_SIZE_IN_PIXELS);

        for i in 0..SCREEN_SIZE_IN_PIXELS {
            let index = (start_index + i) % SCREEN_SIZE_IN_PIXELS;
            if self.get_pixel_at_index(index) == CLEAR_COLOR {
                self.set_pixel_at_index(index, FOOD_COLOR);
                return;
            }
        }
    }

    fn die(&mut self) {
        self.alive = false;

        for x in 0..SCREEN_WIDTH as i32 {
            self.set_pixel(x, 0, FAIL_COLOR);
            self.set_pixel(x, SCREEN_HEIGHT as i32 - 1, FAIL_COLOR);
        }

        for y in 0..SCREEN_HEIGHT as i32 {
            self.set_pixel(0, y, FAIL_COLOR);
            self.set_pixel(SCREEN_WIDTH as i32 - 1, y, FAIL_COLOR);
        }
    }

    fn set_pixel_at_vec(&mut self, vec2d: &Vec2D, color: Color) {
        self.set_pixel(vec2d.x, vec2d.y, color);
    }

    fn get_pixel_at_vec(&mut self, vec2d: &Vec2D) -> Color {
        self.get_pixel(vec2d.x, vec2d.y)
    }

    fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        let i = World::get_index_at_vec(x, y);
        self.set_pixel_at_index(i, color);
    }

    fn get_pixel(&self, x: i32, y: i32) -> Color {
        let i = World::get_index_at_vec(x, y);
        self.get_pixel_at_index(i)
    }

    fn set_pixel_at_index(&mut self, index: usize, color: Color) {
        let i = index * BYTES_PER_PIXEL;
        self.screen[i..i + 3].copy_from_slice(&color);
    }

    fn get_pixel_at_index(&self, index: usize) -> Color {
        let i = index * BYTES_PER_PIXEL;
        [self.screen[i], self.screen[i + 1], self.screen[i + 2]]
    }

    fn get_index_at_vec(x: i32, y: i32) -> usize {
        (y as usize * SCREEN_WIDTH) + x as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let world = World::new();

        assert_eq!(world.get_pixel(0, START_Y), SNAKE_COLOR);
        assert_eq!(world.get_pixel(START_LEN, START_Y), CLEAR_COLOR);
    }

    #[test]
    fn crawl() {
        let mut world = World::new();
        world.tick();

        assert_eq!(world.get_pixel(0, START_Y), CLEAR_COLOR);
        assert_eq!(world.get_pixel(START_LEN, START_Y), SNAKE_COLOR);
    }

    #[test]
    fn turn() {
        let mut world = World::new();
        world.click(0, 0);
        world.tick();

        assert_eq!(world.get_pixel(0, START_Y), CLEAR_COLOR);
        assert_eq!(world.get_pixel(START_LEN, START_Y), CLEAR_COLOR);
        assert_eq!(world.get_pixel(START_LEN - 1, START_Y - 1), SNAKE_COLOR);
    }

    #[test]
    fn eat() {
        let mut world = World::new();
        world.tick();
        world.click(0, 0);
        world.tick();
        world.tick();

        assert_eq!(world.snake.len(), START_LEN as usize + 1);
        assert!(food_exists(&world));
    }

    #[test]
    fn die() {
        let mut world = World::new();
        world.click(0, 0);
        world.tick();
        world.click(0, 0);
        world.tick();
        world.click(0, SCREEN_HEIGHT as i32 - 1);
        world.tick();

        assert_eq!(world.get_pixel(0, 0), FAIL_COLOR);
    }

    #[test]
    fn stop() {
        let mut world = World::new();
        world.click(0, 0);
        world.tick();
        world.click(0, 0);
        world.tick();
        world.click(0, SCREEN_HEIGHT as i32 - 1);
        world.tick();
        world.tick();

        assert_eq!(world.get_pixel(2, START_Y), SNAKE_COLOR);
    }

    #[test]
    fn revive() {
        let mut world = World::new();
        world.click(0, 0);
        world.tick();
        world.click(0, 0);
        world.tick();
        world.click(0, SCREEN_HEIGHT as i32 - 1);
        world.tick();
        world.click(0, 0);

        assert_eq!(world.get_pixel(0, 0), CLEAR_COLOR);
    }

    #[test]
    fn wrap() {
        let mut world = World::new();

        for _ in 0..SCREEN_WIDTH {
            world.tick();
        }

        assert_eq!(world.get_pixel(0, START_Y), SNAKE_COLOR);
    }

    fn food_exists(world: &World) -> bool {
        for i in 0..SCREEN_SIZE_IN_PIXELS {
            if world.get_pixel_at_index(i) == FOOD_COLOR {
                println!("{i}");
                return true;
            }
        }

        false
    }
}
