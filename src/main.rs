extern crate nalgebra_glm as glm;

use crate::plane::Plane;
use crate::utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

mod background;
mod bit_set;
mod camera;
mod cover;
mod flight;
mod floor;
mod model;
mod orientation;
mod path;
mod plane;
mod plane_geometry;
mod plane_program;
mod scene;
mod smooth;
mod utils;

#[wasm_bindgen(inline_js = r"
function nextFrame() {
    return new Promise(resolve => requestAnimationFrame(resolve));
}

export function getCanvas() {
    return document.getElementById('canvas');
}

export async function animationLoop(callback) {
    let startTime = performance.now();
    const container = document.getElementById('container');
    const canvas = document.getElementById('canvas');

    function resize() {
        const w = container.clientWidth;
        const h = container.clientHeight;
        if (canvas.clientWidth !== w || canvas.clientHeight !== h) {
            canvas.style.width = `${w}px`;
            canvas.style.height = `${h}px`;
            canvas.width = w * devicePixelRatio;
            canvas.height = h * devicePixelRatio;
        }
    }

    function resizeWithDelay() {
        resize();
        setTimeout(resize, 200);
    }

    window.addEventListener('resize', resizeWithDelay);
    window.addEventListener('orientationchange', resizeWithDelay);
    resizeWithDelay();

    // noinspection InfiniteLoopJS
    while (true) {
        const w = container.clientWidth * devicePixelRatio;
        const h = container.clientHeight * devicePixelRatio;
        callback(w, h, (performance.now() - startTime) / 40000 % 1.0);
        await nextFrame();
    }
}
")]
extern "C" {
    #[wasm_bindgen(js_name = animationLoop)]
    fn animation_loop(callback: &Closure<dyn Fn(i32, i32, f32)>);
    #[wasm_bindgen(js_name = getCanvas)]
    fn get_canvas() -> HtmlCanvasElement;
}

fn main() {
    set_panic_hook();
    let plane = Plane::new(&get_canvas()).unwrap();
    let callback = Box::new(Closure::wrap(Box::new(move |w: i32, h: i32, phase: f32| {
        plane.render(w, h, phase);
    }) as Box<dyn Fn(i32, i32, f32)>));
    animation_loop(&callback);
    callback.forget();
}
