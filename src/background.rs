use glm::Vec2;
use webgl_rc::{
    load_glsl, Attributes, BufferUsage, Gl, GlError, ItemsBuffer, PrimitiveType, Program, Settings,
    Uniforms,
};

#[derive(Debug, Clone, Copy, PartialEq, Attributes)]
struct Point {
    position: Vec2,
}

#[derive(Debug, Clone, Copy, PartialEq, Uniforms)]
struct BackgroundUniforms {}

impl From<Vec2> for Point {
    fn from(position: Vec2) -> Self {
        Self { position }
    }
}

pub struct Background {
    gl: Gl,
    program: Program,
    points: ItemsBuffer<Point>,
}

impl Background {
    pub fn new(gl: Gl) -> Result<Self, GlError> {
        let program = gl.program(
            load_glsl!("background.f.glsl"),
            load_glsl!("background.v.glsl"),
        )?;

        let points = ItemsBuffer::new(
            gl.clone(),
            &[
                Vec2::new(-1.0, -1.0).into(),
                Vec2::new(1.0, -1.0).into(),
                Vec2::new(1.0, 1.0).into(),
                Vec2::new(-1.0, 1.0).into(),
            ],
            BufferUsage::Static,
        )?;

        Ok(Self {
            gl,
            program,
            points,
        })
    }

    pub fn render(&self) {
        self.gl
            .apply(Gl::settings().depth_test(false).blend(false), || {
                self.program.draw_arrays(
                    PrimitiveType::TriangleFan,
                    &BackgroundUniforms {},
                    &self.points,
                );
            });
    }
}
