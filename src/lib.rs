mod world;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};
use world::{World, SCREEN_HEIGHT, SCREEN_WIDTH};

#[wasm_bindgen]
pub struct Game {
    world: World,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        Game {
            world: World::new(),
        }
    }

    pub fn width() -> usize {
        SCREEN_WIDTH
    }

    pub fn height() -> usize {
        SCREEN_HEIGHT
    }

    pub fn tick(&mut self) {}

    pub fn render(&mut self, ctx: &CanvasRenderingContext2d) {
        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&self.world.screen),
            SCREEN_WIDTH.try_into().unwrap(),
            SCREEN_HEIGHT.try_into().unwrap(),
        )
        .expect("should create ImageData from array");

        ctx.put_image_data(&data, 0.0, 0.0)
            .expect("should write array to context");
    }

    pub fn turn(&mut self) {}
}
