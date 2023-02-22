use std::cmp;
use std::f64;
use wasm_bindgen::prelude::*;
mod snake;
use snake::{Point2D, Snake};

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn canvas() -> web_sys::HtmlCanvasElement {
    document()
        .get_element_by_id("canvas")
        .expect("should have an element with id 'canvas' in the document")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .expect("canvas should be a HtmlCanvasElement")
}

#[wasm_bindgen]
pub struct Game {
    snake: Snake,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        Game {
            snake: Snake::new(),
        }
    }

    pub fn tick(&mut self) -> bool {
        self.snake.move_forward()
    }

    pub fn render(&self) {
        let canvas = canvas();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

        context.set_fill_style(&JsValue::from_str("green"));

        for segment in self.snake.segments() {
            let v0 = &segment.0;
            let v1 = &segment.1;

            let x = if v0.x != v1.x {
                cmp::min(v0.x, v1.x)
            } else {
                v0.x - 2
            };
            let y = if v0.y != v1.y {
                cmp::min(v0.y, v1.y)
            } else {
                v0.y - 2
            };
            let w = cmp::max(i32::abs_diff(v0.x, v1.x), 4);
            let h = cmp::max(i32::abs_diff(v0.y, v1.y), 4);

            context.set_fill_style(&JsValue::from_str("green"));
            context.fill_rect(x as f64, y as f64, w as f64, h as f64);
            context.begin_path();
            context
                .arc(v0.x as f64, v0.y as f64, 2.0, 0.0, f64::consts::PI * 2.0)
                .unwrap();
            context
                .arc(v1.x as f64, v1.y as f64, 2.0, 0.0, f64::consts::PI * 2.0)
                .unwrap();
            context.set_fill_style(&JsValue::from_str("red"));
            context
                .arc(
                    self.snake.food.x as f64,
                    self.snake.food.y as f64,
                    2.0,
                    0.0,
                    f64::consts::PI * 2.0,
                )
                .unwrap();
            context.fill();
        }
    }

    pub fn turn(&mut self) {
        self.snake.turn(snake::TurnDirection::Left)
    }
}
