#![allow(dead_code)]

extern crate nalgebra_glm as glm;

use glm::Vec3;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use webgl_rc::{Gl, GlError, Settings};

use crate::flight::Flight;
use crate::plane_geometry::PlaneGeometry;
use crate::plane_program::PlaneProgram;
use crate::scene::Scene;
use crate::utils::set_panic_hook;

mod bezier;
mod bit_set;
mod camera;
mod flight;
mod mix;
mod model;
mod plane_geometry;
mod plane_orientation;
mod plane_program;
mod scene;
mod utils;

const DURATION: f64 = 6000.0;

const LIGHT_POSITION: Vec3 = Vec3::new(3.0, 0.5, -3.0);

#[wasm_bindgen(start)]
pub fn start() {
    set_panic_hook();
}

#[wasm_bindgen]
pub struct Plane {
    gl: Gl,
    plane_program: PlaneProgram,
    plane_geometry: PlaneGeometry,
    flight: Flight,
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
            flight: Flight::new(),
        })
    }

    pub fn render(&self, w: i32, h: i32, phase: f32) {
        let ratio = (w as f32) / (h as f32);
        let frame = self.flight.get(phase);
        let scene = Scene {
            camera: frame.get_camera(ratio),
            light_position: LIGHT_POSITION,
            model_matrix: frame.get_model_matrix(),
        };

        self.gl.apply(
            Gl::settings()
                .clear_color(0.0, 0.0, 0.0, 1.0)
                .clear_depth(1.0)
                .viewport(0, 0, w, h),
            || {
                self.gl.clear_buffers();
                self.plane_program
                    .draw(&scene, &self.plane_geometry.get_model(frame.fold_phase))
            },
        );
    }
}
