mod coord;

use self::coord::Coord;
use rand::Rng;
use std::collections::VecDeque;
use std::iter;
use std::ops::{Index, IndexMut};

const BYTES_PER_PIXEL: u32 = 4;

type Color = [u8; 3];
pub const CLEAR_COLOR: Color = [0, 0, 0];
pub const SNAKE_COLOR: Color = [0, 255, 0];
pub const FOOD_COLOR: Color = [0, 0, 255];
pub const FAIL_COLOR: Color = [255, 0, 0];

const START_LEN: i32 = 7;
const START_Y: i32 = 30 as i32 / 2; // todo: this shouldn't be a const
const INVARIANT: &str = "Snake length > 0";


pub struct World {
    pub screen: Screen,
    direction: Coord,
    snake: VecDeque<Coord>,
    alive: bool,
}

impl World {
    pub fn new(width: u32, height: u32) -> World {

        let mut world = World {
            screen: Screen::new(width, height),
            direction: (1, 0).into(),
            snake: VecDeque::new(),
            alive: true,
        };

        world.screen.clear();
        world.create_initial_snake();
        world.create_initial_food();
        world
    }



    pub fn tick(&mut self) {
        if self.alive {
            let new_head = self.get_new_head();
            let new_head_pixel = self.screen.get_color_at(&new_head);

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
                0 => (if x < head.x { -1 } else { 1 }, 0),
                _ => (0, if y < head.y { -1 } else { 1 }),
            }.into();

        } else {
            self.direction = (1, 0).into();
            self.snake = VecDeque::new();
            self.alive = true;
            self.screen.clear();
            self.create_initial_snake();
            self.create_initial_food();
        }
    }

    fn create_initial_snake(&mut self) {
        for x in 0..START_LEN {
            self.screen.set_color_at(&(x, START_Y).into(), SNAKE_COLOR);
            self.snake.push_back((x, START_Y).into());
        }
    }

    fn create_initial_food(&mut self) {
        self.screen.set_color_at(&(START_LEN, START_Y - 2).into(), FOOD_COLOR);
    }

    fn get_new_head(&self) -> Coord {
        let screen_width = self.screen.width;
        let screen_height = self.screen.height;
        let moved_head = *self.snake.back().expect(INVARIANT) + self.direction;
        let x = (moved_head.x + screen_width as i32) % screen_width as i32;
        let y = (moved_head.y + screen_height as i32) % screen_height as i32;
        (x, y).into()
    }

    fn extend_head_to(&mut self, new_head: &Coord) {
        self.screen.set_color_at(new_head, SNAKE_COLOR);
        self.snake.push_back(*new_head);
    }

    fn shorten_tail(&mut self) {
        let tail = self.snake.pop_front().expect(INVARIANT);
        self.screen.set_color_at(&tail, CLEAR_COLOR);
    }

    fn create_food(&mut self) {
        let pixel_count = self.screen.pixel_count as usize;
        let random_skip = rand::thread_rng().gen_range(0..pixel_count) as usize;

        // todo: this will panic if a player fills the screen with the snake ... instead a "you are awesome" message should appear :)
        // todo: this is less efficient than the previous solution ... there might be a better way, but still use coords instead of index

        let coord  = self.screen.iter_pixels()
            .filter(|(color, _)| *color == CLEAR_COLOR)
            .map(|(_, coord)| coord)
            .collect::<Vec<_>>().into_iter()
            .cycle()
            .skip(random_skip)
            .next()
            .expect("At least one pixel should be free.");

        self.screen.set_color_at(&coord, FOOD_COLOR);
    }

    fn die(&mut self) {
        self.alive = false;
        let screen_width = self.screen.width;
        let screen_height = self.screen.height;

        for x in 0..screen_width as i32 {
            self.screen.set_color_at(&(x, 0).into(), FAIL_COLOR);
            self.screen.set_color_at(&(x, screen_height as i32 - 1).into(), FAIL_COLOR);
        }

        for y in 0..screen_height as i32 {
            self.screen.set_color_at(&(0, y).into(), FAIL_COLOR);
            self.screen.set_color_at(&(screen_width as i32 - 1, y).into(), FAIL_COLOR);
        }
    }


}




pub struct Screen {
    pub pixel_buffer: Vec<u8>,
    pub pixel_count: u32,
    pub width: u32,
    pub height: u32,
}

impl Screen {

    fn new(width: u32, height: u32) -> Self {
        let pixel_count = width * height;
        let screen_size_in_bytes = pixel_count * BYTES_PER_PIXEL;
        Self {
            pixel_count, width, height,
            pixel_buffer: vec![255u8; screen_size_in_bytes as usize],
        }
    }

    fn clear(&mut self) {
        for x in 0..self.width as i32 {
            for y in 0..self.height as i32 {
                self.set_color_at(&(x, y).into(), CLEAR_COLOR);
            }
        }
    }

    fn set_color_at(&mut self, coord: &Coord, color: Color) {
        let i = self.get_buffer_index_for(coord);
        self.pixel_buffer[i..i + 3].copy_from_slice(&color);
    }

    fn get_color_at(&self, coord: &Coord) -> Color {
        let i = self.get_buffer_index_for(coord);
        [self.pixel_buffer[i], self.pixel_buffer[i + 1], self.pixel_buffer[i + 2]]
    }

    fn get_buffer_index_for(&self, Coord{x, y}: &Coord) -> usize {
        *y as usize * self.width as usize + *x as usize * BYTES_PER_PIXEL as usize
    }

    fn iter_pixels(&self) -> impl Iterator<Item=(Color, Coord)> + '_{
        (0..self.width as i32).flat_map(|x|
            (0..self.height as i32).map(move |y| (x, y).into()))
            .map(|coord: Coord| {
                (self.get_color_at(&coord), coord)
            })
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let world = World::new(30, 30);

        assert_eq!(world.screen.get_color_at(&(0, START_Y).into()), SNAKE_COLOR);
        assert_eq!(world.screen.get_color_at(&(START_LEN, START_Y).into()), CLEAR_COLOR);
    }

    #[test]
    fn crawl() {
        let mut world = World::new(30, 30);
        world.tick();

        assert_eq!(world.screen.get_color_at(&(0, START_Y).into()), CLEAR_COLOR);
        assert_eq!(world.screen.get_color_at(&(START_LEN, START_Y).into()), SNAKE_COLOR);
    }

    #[test]
    fn turn() {
        let mut world = World::new(30, 30);
        world.click(0, 0);
        world.tick();

        assert_eq!(world.screen.get_color_at(&(0, START_Y).into()), CLEAR_COLOR);
        assert_eq!(world.screen.get_color_at(&(START_LEN, START_Y).into()), CLEAR_COLOR);
        assert_eq!(world.screen.get_color_at(&(START_LEN - 1, START_Y - 1).into()), SNAKE_COLOR);
    }

    #[test]
    fn eat() {
        let mut world = World::new(30, 30);
        world.tick();
        world.click(0, 0);
        world.tick();
        world.tick();

        assert_eq!(world.snake.len(), START_LEN as usize + 1);
        assert!(food_exists(&world));
    }

    #[test]
    fn die() {
        let mut world = World::new(30, 30);
        world.click(0, 0);
        world.tick();
        world.click(0, 0);
        world.tick();
        world.click(0, world.screen.height as i32 - 1);
        world.tick();

        assert_eq!(world.screen.get_color_at(&(0, 0).into()), FAIL_COLOR);
    }

    #[test]
    fn stop() {
        let mut world = World::new(30, 30);
        world.click(0, 0);
        world.tick();
        world.click(0, 0);
        world.tick();
        world.click(0, world.screen.height as i32 - 1);
        world.tick();
        world.tick();

        assert_eq!(world.screen.get_color_at(&(2, START_Y).into()), SNAKE_COLOR);
    }

    #[test]
    fn revive() {
        let mut world = World::new(30, 30);
        world.click(0, 0);
        world.tick();
        world.click(0, 0);
        world.tick();
        world.click(0, world.screen.height as i32 - 1);
        world.tick();
        world.click(0, 0);

        assert_eq!(world.screen.get_color_at(&(0, 0).into()), CLEAR_COLOR);
    }

    #[test]
    fn wrap() {
        let mut world = World::new(30, 30);

        for _ in 0..world.screen.width {
            world.tick();
        }

        assert_eq!(world.screen.get_color_at(&(0, START_Y).into()), SNAKE_COLOR);
    }

    fn food_exists(world: &World) -> bool {
        world.screen.iter_pixels().any(|(color, _)| color == FOOD_COLOR)
    }
}
