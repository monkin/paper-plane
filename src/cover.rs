use glm::Vec2;
use webgl_rc::{
    load_glsl, Attributes, BlendFunction, BufferUsage, Gl, GlError, ItemsBuffer, PrimitiveType,
    Program, Settings, Uniforms,
};

#[derive(Clone, Copy, Debug, PartialEq, Uniforms)]
struct CoverUniforms {
    opacity: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Attributes)]
struct CoverAttributes {
    position: Vec2,
}

impl From<Vec2> for CoverAttributes {
    fn from(position: Vec2) -> CoverAttributes {
        CoverAttributes { position }
    }
}

pub struct Cover {
    gl: Gl,
    program: Program,
    vertices: ItemsBuffer<CoverAttributes>,
}

impl Cover {
    pub fn new(gl: Gl) -> Result<Cover, GlError> {
        let program = gl.program(load_glsl!("cover.f.glsl"), load_glsl!("cover.v.glsl"))?;
        let vertices = gl.items_buffer(
            &[
                Vec2::new(-1.0, -1.0).into(),
                Vec2::new(1.0, -1.0).into(),
                Vec2::new(1.0, 1.0).into(),
                Vec2::new(-1.0, 1.0).into(),
            ],
            BufferUsage::Static,
        )?;
        Ok(Cover {
            gl,
            program,
            vertices,
        })
    }

    pub fn render(&self, opacity: f32) {
        if opacity > 0.0 {
            self.gl.apply(
                Gl::settings().depth_test(false).blend(true).blend_function(
                    BlendFunction::One,
                    BlendFunction::OneMinusSrcAlpha,
                    BlendFunction::One,
                    BlendFunction::OneMinusSrcAlpha,
                ),
                || {
                    self.program.draw_arrays(
                        PrimitiveType::TriangleFan,
                        &CoverUniforms { opacity },
                        &self.vertices,
                    );
                },
            );
        }
    }
}
