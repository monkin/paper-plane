use std::iter::once;

use glm::{Mat4, Vec3};
use webgl_rc::{
    Attributes, BufferUsage, Gl, GlError, Instances, ItemsBuffer, load_glsl, PrimitiveType,
    Program, Uniforms,
};

use crate::camera::Camera;
use crate::plane_geometry::{PlaneGeometry, PlanePrimitives};

#[derive(Clone, Debug)]
pub struct PlaneProgram {
    gl: Gl,
    geometry: PlaneGeometry,
    triangles_program: Program,
    lines_program: Program,
    lines_array: ItemsBuffer<LineVertex>,
    triangles_array: ItemsBuffer<TriangleVertex>,
    sides_array: ItemsBuffer<SideAttribute>,
}

#[derive(Clone, Copy, PartialEq, Debug, Attributes)]
struct LineVertex {
    position: Vec3,
    opacity: f32,
}

#[derive(Clone, Copy, PartialEq, Debug, Instances)]
struct SideAttribute {
    side: f32,
}

#[derive(Clone, Copy, PartialEq, Debug, Uniforms)]
struct LineUniforms {
    camera: Mat4,
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
        let sides_array = gl.items_buffer(
            &vec![SideAttribute { side: 1.0 }, SideAttribute { side: -1.0 }],
            BufferUsage::Dynamic,
        )?;

        Ok(PlaneProgram {
            gl,
            geometry,
            lines_program,
            triangles_program,
            lines_array,
            triangles_array,
            sides_array,
        })
    }

    pub fn draw(&self, camera: &Camera, primitives: &PlanePrimitives) {
        let lines: Vec<_> = primitives
            .lines
            .iter()
            .copied()
            .flat_map(|(p1, p2, a)| {
                once(LineVertex {
                    position: p1,
                    opacity: a,
                })
                .chain(once(LineVertex {
                    position: p2,
                    opacity: a,
                }))
            })
            .collect();

        self.lines_array.set_content(&lines, BufferUsage::Dynamic);
        self.lines_program.draw_instances(
            PrimitiveType::Lines,
            &LineUniforms {
                camera: camera.get_matrix(),
            },
            &self.lines_array,
            &self.sides_array,
        );
    }
}
