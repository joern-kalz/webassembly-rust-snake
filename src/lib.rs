use std::cmp;
use wasm_bindgen::prelude::*;

mod snake;
use snake::Snake;

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
        Game { snake: Snake::new() }
    }

    pub fn tick(& mut self) {
        self.snake.move_forward();
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

        let p = self.snake.segments();

        for n in 1..(p.len()) {
            let v0 = &p[n - 1];
            let v1 = &p[n];

            let x = cmp::min(v0.x, v1.x);
            let y = cmp::min(v0.y, v1.y);
            let w = cmp::max(i32::abs_diff(v0.x, v1.x), 2);
            let h = cmp::max(i32::abs_diff(v0.y, v1.y), 2);

            context.set_fill_style(&JsValue::from_str("green"));
            context.fill_rect(x as f64, y as f64, w as f64, h as f64);
        }
    }

    pub fn turn(& mut self) {
        self.snake.turn(snake::TurnDirection::Left)
    }
}
