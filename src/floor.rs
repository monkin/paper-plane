use glm::{Mat4, Vec2, Vec3};
use webgl_rc::{
    load_glsl, Attributes, BlendFunction, BufferUsage, CullFace, Gl, GlError, ItemsBuffer,
    PrimitiveType, Program, Settings, Uniforms,
};

#[derive(Debug, Clone, Copy, PartialEq, Attributes)]
struct Point {
    position: Vec3,
    texture: Vec2,
}

impl Point {
    pub fn new(position: Vec3, texture: Vec2) -> Self {
        Self { position, texture }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Uniforms)]
struct FloorUniform {
    camera: Mat4,
}

pub struct Floor {
    gl: Gl,
    program: Program,
    points: ItemsBuffer<Point>,
}

impl Floor {
    pub fn new(gl: Gl) -> Result<Self, GlError> {
        let program = gl.program(load_glsl!("floor.f.glsl"), load_glsl!("floor.v.glsl"))?;

        let points = ItemsBuffer::new(gl.clone(), &[], BufferUsage::Dynamic)?;

        Ok(Self {
            gl,
            program,
            points,
        })
    }

    pub fn render(&self, camera: Mat4) {
        let points = [
            Point::new(Vec3::new(-5.0, -1.5, -5.0), Vec2::new(-1.0, -1.0)),
            Point::new(Vec3::new(5.0, -1.5, -5.0), Vec2::new(1.0, -1.0)),
            Point::new(Vec3::new(5.0, -1.5, 5.0), Vec2::new(1.0, 1.0)),
            Point::new(Vec3::new(-5.0, -1.5, 5.0), Vec2::new(-1.0, 1.0)),
        ];

        self.points.set_content(&points, BufferUsage::Dynamic);

        self.gl.apply(
            Gl::settings()
                .depth_test(false)
                .blend(true)
                .blend_function(
                    BlendFunction::One,
                    BlendFunction::OneMinusSrcAlpha,
                    BlendFunction::One,
                    BlendFunction::OneMinusSrcAlpha,
                )
                .cull_face(CullFace::FrontAndBack),
            || {
                self.program.draw_arrays(
                    PrimitiveType::TriangleFan,
                    &FloorUniform { camera },
                    &self.points,
                );
            },
        );
    }
}
