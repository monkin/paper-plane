use std::f32::consts::PI;

use glm::Vec3;

use crate::bit_set::BitSet;
use crate::fold::Fold;

const SCALE: f32 = 1.0 / 297.0;

static POINTS: &[(f32, f32)] = &[
    (0.0, 0.0),
    (0.0, 105.0),
    (105.0, 0.0),
    (105.0, 105.0),
    (0.0, 297.0),
    (13.0, 105.0),
    (105.0, 211.064),
    (55.733, 55.733),
    (13.0, 0.0),
    (13.0, 13.0),
    (0.0, 13.0),
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

struct Step {
    duration: f32,
    transformation: Fold,
    /// Lines to appear during this step
    lines: &'static [(u8, u8)],
}

static STEPS: &[Step] = &[
    Step {
        duration: 1.0,
        transformation: Fold::new(
            (0, 4),
            BitSet::with_bits(&[2, 3, 5, 6, 7, 8, 9, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]),
            PI * 0.5,
        ),
        lines: &[
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
        ],
    },
    Step {
        duration: 1.0,
        transformation: Fold::new(
            (0, 4),
            BitSet::with_bits(&[
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
            ]),
            -PI * 0.5,
        ),
        lines: &[],
    },
    Step {
        duration: 1.0,
        transformation: Fold::new((0, 3), BitSet::with_bits(&[8, 2, 14, 19, 18]), PI),
        lines: &[(0, 9), (9, 13), (13, 7), (7, 17), (17, 3)],
    },
    Step {
        duration: 1.0,
        transformation: Fold::new(
            (1, 3),
            BitSet::with_bits(&[0, 8, 2, 14, 19, 18, 9, 13, 7, 17, 10]),
            -PI,
        ),
        lines: &[(1, 11), (11, 5), (5, 15), (15, 3)],
    },
    Step {
        duration: 1.0,
        transformation: Fold::new((5, 6), BitSet::with_bits(&[18, 17, 3, 15, 16]), -PI),
        lines: &[(5, 6), (5, 7), (7, 19)],
    },
    Step {
        duration: 1.0,
        transformation: Fold::new((9, 10), BitSet::with_bits(&[0]), -PI),
        lines: &[(9, 10), (9, 8)],
    },
    Step {
        duration: 1.0,
        transformation: Fold::new(
            (1, 4),
            BitSet::with_bits(&[2, 3, 5, 6, 7, 8, 9, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]),
            -PI * 0.5,
        ),
        lines: &[],
    },
    Step {
        duration: 1.0,
        transformation: Fold::new((11, 12), BitSet::with_bits(&[5, 7, 19, 6, 20]), -PI * 0.5),
        lines: &[(11, 12), (11, 13), (13, 14), (15, 16), (15, 17), (17, 18)],
    },
];

pub struct PlanePrimitives {
    pub lines: Vec<(Vec3, Vec3, f32)>,
    pub triangles: Vec<(Vec3, Vec3, Vec3)>,
}

#[derive(Clone, Debug)]
pub struct PlaneGeometry {
    total_duration: f32,
    /// Input points for every step
    input_points: Vec<Vec<Vec3>>,
}

/// Animated foldable plane geometry
impl PlaneGeometry {
    pub fn new() -> PlaneGeometry {
        PlaneGeometry {
            total_duration: STEPS
                .iter()
                .map(|s| s.duration)
                .reduce(|r, v| r + v)
                .unwrap_or(0.0),
            input_points: {
                let mut points: Vec<Vec<Vec3>> = Vec::with_capacity(STEPS.len());
                let offset = Vec3::new(0.0, -0.5, 0.0);
                for i in 0..STEPS.len() {
                    points.push(if i == 0 {
                        POINTS
                            .iter()
                            .map(|(x, y)| Vec3::new(*x, *y, 0.0) * SCALE + offset)
                            .collect()
                    } else {
                        STEPS[i - 1].transformation.apply(&points[i - 1], 1.0)
                    });
                }
                points
            },
        }
    }

    /// Get points for time from 0.0 to 1.0
    pub fn get_primitives(&self, t: f32) -> PlanePrimitives {
        let mut time = t.clamp(0.0, 1.0) * self.total_duration;

        let mut index: usize = 0;
        for (i, s) in STEPS.iter().enumerate() {
            index = i;
            if time <= s.duration || i == STEPS.len() - 1 {
                break;
            } else {
                time -= s.duration;
            }
        }

        let step = &STEPS[index];
        let m = time / step.duration;

        let points = step.transformation.apply(&self.input_points[index], m);

        let lines_opacity = t;

        PlanePrimitives {
            lines: (0..index)
                .flat_map(|i| STEPS[i].lines.iter().copied().map(|(p1, p2)| (p1, p2, 1.0)))
                .chain(
                    step.lines
                        .iter()
                        .copied()
                        .map(|(p1, p2)| (p1, p2, lines_opacity)),
                )
                .map(|(p1, p2, opacity)| (points[p1 as usize], points[p2 as usize], opacity))
                .collect(),
            triangles: INDEXES
                .iter()
                .copied()
                .map(|(i1, i2, i3)| {
                    (
                        points[i1 as usize],
                        points[i2 as usize],
                        points[i3 as usize],
                    )
                })
                .collect(),
        }
    }
}
