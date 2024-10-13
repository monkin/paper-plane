#![allow(dead_code)]

extern crate nalgebra_glm as glm;

use std::f32::consts::PI;

use glm::Vec3;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use webgl_rc::{Gl, GlError, Item, Settings};

use crate::camera::Camera;
use crate::plane_geometry::PlaneGeometry;
use crate::plane_program::PlaneProgram;
use crate::utils::set_panic_hook;

mod bezier;
mod bit_set;
mod camera;
mod flight;
mod fold;
mod mix;
mod plane_geometry;
mod plane_program;
mod utils;

const DURATION: f64 = 6000.0;

#[wasm_bindgen(start)]
pub fn start() {
    set_panic_hook();
}

#[wasm_bindgen]
pub struct Plane {
    gl: Gl,
    plane_program: PlaneProgram,
    plane_geometry: PlaneGeometry,
}

#[wasm_bindgen]
impl Plane {
    #[wasm_bindgen(constructor, catch)]
    pub fn new(canvas: &HtmlCanvasElement) -> Result<Plane, GlError> {
        let plane_geometry = PlaneGeometry::new();
        let gl = Gl::new(canvas)?;
        let plane_program = PlaneProgram::new(gl.clone())?;

        Ok(Plane {
            gl,
            plane_program,
            plane_geometry,
        })
    }

    pub fn render(&self, w: i32, h: i32, phase: f32) {
        let ratio = (w as f32) / (h as f32);
        let camera = Camera {
            position: Vec3::new(4.0, -6.0, 5.0),
            target: Vec3::new(0.0, 0.0, 0.0),
            fov: 80.0 / (2.0 * PI),
            width: 2.0 * ratio,
            height: 2.0 / ratio,
            far: 3.0,
            near: -3.0,
        };
        self.gl.apply(
            Gl::settings()
                .clear_color(0.5, 1.0, 1.0, 1.0)
                .viewport(0, 0, w, h),
            || {
                self.gl.clear_buffers();
                self.plane_program
                    .draw(&camera, &self.plane_geometry.get_primitives(phase))
            },
        );
    }
}
