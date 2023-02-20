use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

mod snake;
use snake::Point2D;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

// This function is automatically invoked after the wasm module is instantiated.
#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let snake = vec![Point2D(5,1), Point2D(10,1), Point2D(10,20) ];
    *g.borrow_mut() = Some(Closure::new(move || {
        let canvas = document().get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        context.set_fill_style(&JsValue::from_str("green"));

        for n in 1..(snake.len()) {
            let v0 = &snake[n - 1];
            let v1 = &snake[n];

            if v0.0 == v1.0 {
                let x = 
                context.set_fill_style(&JsValue::from_str("green"));
                context.fill_rect(v0.0 as f64, v0.1 as f64,  2.0, (v1.1 - v0.1) as f64);
            } else {
                context.set_fill_style(&JsValue::from_str("red"));
                context.fill_rect(v0.0 as f64, v0.1 as f64,(v1.0 - v0.0) as f64, 2.0);
            }
        }

        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}