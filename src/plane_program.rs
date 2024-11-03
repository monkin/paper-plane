use std::iter::once;

use crate::model::Model;
use crate::plane_geometry::PlaneGeometry;
use crate::scene::Scene;
use glm::{Mat4, Vec3};
use webgl_rc::{
    load_glsl, Attributes, BlendFunction, BufferUsage, CullFace, DepthFunction, Gl, GlError,
    ItemsBuffer, PrimitiveType, Program, Settings, Uniforms,
};

pub struct PlaneProgram {
    gl: Gl,
    geometry: PlaneGeometry,
    triangles_program: Program,
    lines_program: Program,
    lines_array: ItemsBuffer<LineVertex>,
    triangles_array: ItemsBuffer<TriangleVertex>,
}

#[derive(Clone, Copy, PartialEq, Debug, Attributes)]
struct LineVertex {
    position: Vec3,
    opacity: f32,
}

#[derive(Clone, Copy, PartialEq, Debug, Uniforms)]
struct LineUniforms {
    camera: Mat4,
}

#[derive(Clone, Copy, PartialEq, Debug, Uniforms)]
struct TriangleUniforms {
    camera: Mat4,
    light_position: Vec3,
}

#[derive(Clone, Copy, PartialEq, Debug, Attributes)]
struct TriangleVertex {
    position: Vec3,
    normal: Vec3,
}

struct PlaneUniforms {}

impl PlaneProgram {
    pub fn new(gl: Gl) -> Result<PlaneProgram, GlError> {
        let geometry = PlaneGeometry::new();
        let lines_program = gl.program(
            load_glsl!("plane-line.f.glsl"),
            load_glsl!("plane-line.v.glsl"),
        )?;
        let triangles_program = gl.program(
            load_glsl!("plane-triangle.f.glsl"),
            load_glsl!("plane-triangle.v.glsl"),
        )?;
        let triangles_array: ItemsBuffer<TriangleVertex> =
            gl.items_buffer(&[], BufferUsage::Dynamic)?;
        let lines_array: ItemsBuffer<LineVertex> = gl.items_buffer(&[], BufferUsage::Dynamic)?;

        Ok(PlaneProgram {
            gl,
            geometry,
            lines_program,
            triangles_program,
            lines_array,
            triangles_array,
        })
    }

    pub fn draw(&self, scene: &Scene, model: &Model) {
        let camera = &scene.camera;
        let plane_matrix = camera.get_projection_matrix() * camera.get_view_matrix();
        let light_position = scene.light_position;

        let vertices: Vec<_> = model
            .vertices
            .iter()
            .map(|v| scene.model_matrix * v.push(1.0))
            .map(|v| v.xyz())
            .collect();

        let triangles: Vec<_> = model
            .triangles
            .iter()
            .copied()
            .map(|(a, b, c)| {
                let p1 = vertices[a as usize];
                let p2 = vertices[b as usize];
                let p3 = vertices[c as usize];
                (p1, p2, p3)
            })
            .flat_map(|(p1, p2, p3)| {
                let normal: Vec3 = (p2 - p1).cross(&(p3 - p1)).normalize();
                once(TriangleVertex {
                    position: p1,
                    normal,
                })
                .chain(once(TriangleVertex {
                    position: p2,
                    normal,
                }))
                .chain(once(TriangleVertex {
                    position: p3,
                    normal,
                }))
            })
            .collect();

        self.triangles_array
            .set_content(&triangles, BufferUsage::Dynamic);

        let lines: Vec<_> = model
            .lines
            .iter()
            .copied()
            .flat_map(|(p1, p2, a)| {
                once(LineVertex {
                    position: vertices[p1 as usize],
                    opacity: a,
                })
                .chain(once(LineVertex {
                    position: vertices[p2 as usize],
                    opacity: a,
                }))
            })
            .collect();

        self.lines_array.set_content(&lines, BufferUsage::Dynamic);

        self.gl.apply(
            Gl::settings()
                .depth_test(true)
                .depth_function(DepthFunction::LEqual)
                .cull_face(CullFace::FrontAndBack),
            || {
                self.triangles_program.draw_arrays(
                    PrimitiveType::Triangles,
                    &TriangleUniforms {
                        camera: plane_matrix,
                        light_position,
                    },
                    &self.triangles_array,
                );
            },
        );

        self.gl.apply(
            Gl::settings().depth_test(false).blend(true).blend_function(
                BlendFunction::One,
                BlendFunction::OneMinusSrcAlpha,
                BlendFunction::One,
                BlendFunction::OneMinusSrcAlpha,
            ),
            || {
                self.lines_program.draw_arrays(
                    PrimitiveType::Lines,
                    &LineUniforms {
                        camera: plane_matrix,
                    },
                    &self.lines_array,
                );
            },
        );
    }
}
