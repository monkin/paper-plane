use crate::model::transform::Transform;
use std::f32::consts::PI;

use crate::bit_set::BitSet;
use crate::model::transform::fold::Fold;
use crate::model::{AnimatedModel, Model, Stay};
use glm::Vec3;

const SCALE: f32 = 1.0 / 297.0;
const FOLD_FACTOR: f32 = 0.97;

static POINTS: &[(f32, f32)] = &[
    (0.0, 0.0),
    (0.0, 105.0),
    (105.0, 0.0),
    (105.0, 105.0),
    (0.0, 297.0),
    (13.0, 105.0),
    (105.0, 211.064),
    (55.733, 55.733),
    (12.0, 0.0),
    (12.0, 12.0),
    (0.0, 12.0),
    (6.5, 105.0),
    (55.0, 297.0),
    (26.3638, 26.3638),
    (105.0, 6.5),
    (29.608, 105.0),
    (105.0, 135.92),
    (83.073, 83.073),
    (105.0, 29.608),
    (105.0, 13.0),
    (105.0, 297.0),
];

static INDEXES: &[(u8, u8, u8)] = &[
    (0, 8, 9),
    (0, 9, 10),
    (8, 13, 9),
    (8, 2, 13),
    (2, 14, 13),
    (10, 9, 13),
    (10, 13, 1),
    (13, 11, 1),
    (13, 14, 7),
    (14, 19, 7),
    (13, 7, 11),
    (7, 5, 11),
    (7, 19, 18),
    (7, 18, 17),
    (7, 17, 15),
    (7, 15, 5),
    (18, 3, 17),
    (17, 3, 15),
    (15, 3, 16),
    (15, 16, 6),
    (15, 6, 5),
    (11, 5, 6),
    (11, 6, 20),
    (11, 20, 12),
    (1, 11, 12),
    (1, 12, 4),
];

fn create_static_plane() -> Model {
    Model {
        vertices: POINTS
            .iter()
            .copied()
            .map(|(x, y)| Vec3::new(x, y, 0.0) * SCALE + Vec3::new(0.0, -0.5, 0.0))
            .collect(),
        triangles: INDEXES.iter().copied().collect(),
        lines: vec![],
    }
}

fn create_animated_plane() -> impl AnimatedModel {
    let model = create_static_plane();
    let model = model.animate(
        1.0,
        Fold::new(
            (0, 4),
            BitSet::with_bits(&[2, 3, 5, 6, 7, 8, 9, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]),
            PI * 0.5 * FOLD_FACTOR,
        )
        .add_lines(vec![
            (0, 10),
            (10, 1),
            (1, 4),
            (0, 8),
            (8, 2),
            (4, 12),
            (12, 20),
            (2, 14),
            (14, 19),
            (19, 18),
            (18, 3),
            (3, 16),
            (16, 6),
            (6, 20),
        ]),
    );
    let model = model.animate(
        1.0,
        Fold::new(
            (0, 4),
            BitSet::with_bits(&[
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
            ]),
            -PI * 0.5 * FOLD_FACTOR,
        ),
    );
    let model = model.animate(
        1.0,
        Fold::new((0, 3), BitSet::with_bits(&[8, 2, 14, 19, 18]), PI).add_lines(vec![
            (0, 9),
            (9, 13),
            (13, 7),
            (7, 17),
            (17, 3),
        ]),
    );
    let model = model.animate(
        1.0,
        Fold::new(
            (1, 3),
            BitSet::with_bits(&[0, 8, 2, 14, 19, 18, 9, 13, 7, 17, 10]),
            PI,
        )
        .add_lines(vec![(1, 11), (11, 5), (5, 15), (15, 3)]),
    );
    let model = model.animate(
        0.25,
        Stay::new().shift(Vec3::new(0.0, -210.0 * 0.5 * 0.5 / 297.0, 0.0)),
    );
    let model = model.animate(
        1.0,
        Fold::new((5, 6), BitSet::with_bits(&[18, 17, 3, 15, 16]), PI).add_lines(vec![
            (5, 6),
            (5, 7),
            (7, 19),
        ]),
    );
    let model = model.animate(
        1.0,
        Fold::new((9, 10), BitSet::with_bits(&[0]), PI).add_lines(vec![(9, 10), (9, 8)]),
    );
    let model = model.animate(
        1.0,
        Fold::new(
            (1, 4),
            BitSet::with_bits(&[2, 3, 5, 6, 7, 8, 9, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]),
            -PI * 0.5 * FOLD_FACTOR,
        ),
    );
    let model = model.animate(
        1.0,
        Fold::new((11, 12), BitSet::with_bits(&[5, 7, 19, 6, 20]), PI * 0.5).add_lines(vec![
            (11, 12),
            (11, 13),
            (13, 14),
            (15, 16),
            (15, 17),
            (17, 18),
        ]),
    );
    let model = model.stay(3.0);

    model
}

pub struct PlaneGeometry {
    model: Box<dyn AnimatedModel>,
}

/// Animated foldable plane geometry
impl PlaneGeometry {
    pub fn new() -> PlaneGeometry {
        PlaneGeometry {
            model: Box::new(create_animated_plane()),
        }
    }

    /// Get points for time from 0.0 to 1.0
    pub fn get_model(&self, t: f32) -> Model {
        let time = t.clamp(0.0, 1.0) * self.model.duration();
        let model = self.model.get_model(time);
        let model = model.clone().merge(model.flip_x());

        model
    }
}
