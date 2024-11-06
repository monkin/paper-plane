use glm::{Mat4, Vec3};

#[derive(Debug, Clone)]
pub struct Model {
    pub vertices: Vec<Vec3>,
    pub triangles: Vec<(u8, u8, u8)>,
    pub lines: Vec<(u8, u8, f32)>,
}

impl Model {
    pub fn merge(self, other: Self) -> Model {
        let mut vertices = self.vertices;
        let mut triangles = self.triangles;
        let mut lines = self.lines;
        let offset = vertices.len() as u8;
        vertices.extend(other.vertices);
        triangles.extend(
            other
                .triangles
                .iter()
                .map(|(a, b, c)| (a + offset, b + offset, c + offset)),
        );
        lines.extend(
            other
                .lines
                .iter()
                .copied()
                .map(|(a, b, alpha)| (a + offset, b + offset, alpha)),
        );
        Self {
            vertices,
            triangles,
            lines,
        }
    }

    pub fn transform(self, matrix: Mat4) -> Self {
        let mut vertices = self.vertices;
        for i in 0..vertices.len() {
            vertices[i] = glm::vec4_to_vec3(&(matrix * vertices[i].push(1.0)));
        }
        Self { vertices, ..self }
    }

    pub fn flip_x(self) -> Self {
        let matrix = glm::scaling(&Vec3::new(-1.0, 1.0, 1.0));
        self.transform(matrix)
    }
}
