use std::iter::once;

use crate::plane_geometry::{PlaneGeometry, PlanePrimitives};
use crate::scene::Scene;
use glm::{Mat4, Vec3};
use webgl_rc::{
    load_glsl, Attributes, BufferUsage, CullFace, DepthFunction, Gl, GlError, Instances,
    ItemsBuffer, PrimitiveType, Program, Settings, Uniforms,
};

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

    pub fn draw(&self, scene: &Scene, primitives: &PlanePrimitives) {
        let camera = &scene.camera;
        let light_position = scene.light_position;

        let triangles: Vec<_> = primitives
            .triangles
            .iter()
            .copied()
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

        self.gl.apply(
            Gl::settings()
                .depth_test(true)
                .depth_function(DepthFunction::LEqual)
                .cull_face(CullFace::FrontAndBack),
            || {
                self.triangles_program.draw_instances(
                    PrimitiveType::Triangles,
                    &TriangleUniforms {
                        camera: camera.get_matrix(),
                        light_position,
                    },
                    &self.triangles_array,
                    &self.sides_array,
                );
            },
        );

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
