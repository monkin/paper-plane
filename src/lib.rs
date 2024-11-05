#![allow(dead_code)]

extern crate nalgebra_glm as glm;

use crate::background::Background;
use crate::cover::Cover;
use crate::flight::Flight;
use crate::floor::Floor;
use crate::plane_geometry::PlaneGeometry;
use crate::plane_program::PlaneProgram;
use crate::scene::Scene;
use crate::utils::set_panic_hook;
use glm::Vec3;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use webgl_rc::{Gl, GlError, Settings};

mod background;
mod bezier;
mod bit_set;
mod camera;
mod cover;
mod flight;
mod floor;
mod model;
mod orientation;
mod path;
mod plane_geometry;
mod plane_program;
mod scene;
mod smooth;
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
    cover: Cover,
    background: Background,
    floor: Floor,
    flight: Flight,
}

#[wasm_bindgen]
impl Plane {
    #[wasm_bindgen(constructor, catch)]
    pub fn new(canvas: &HtmlCanvasElement) -> Result<Plane, GlError> {
        let plane_geometry = PlaneGeometry::new();
        let gl = Gl::new(canvas)?;
        let plane_program = PlaneProgram::new(gl.clone())?;
        let cover = Cover::new(gl.clone())?;
        let background = Background::new(gl.clone())?;
        let floor = Floor::new(gl.clone())?;
        let flight = Flight::new();

        Ok(Plane {
            gl,
            plane_program,
            plane_geometry,
            cover,
            background,
            floor,
            flight,
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
                self.background.render();
                self.floor
                    .render(scene.camera.get_projection_matrix() * scene.camera.get_view_matrix());
                self.plane_program
                    .draw(&scene, &self.plane_geometry.get_model(frame.fold_phase));
                self.cover.render(frame.cover_opacity);
            },
        );
    }
}
