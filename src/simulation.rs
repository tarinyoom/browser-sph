use std::collections::HashMap;
use std::f32::consts::PI;

use bevy::prelude::{Entity, Vec2};

use crate::types::{Fluid, Particle};

fn smooth(d: f32, r: f32) -> f32 {
    let volume = PI * r.powf(8.0) / 4.0;
    let value = (r * r - d * d).max(0.0);
    500.0 * value * value * value / volume
}

pub fn update_particle(
    id: Entity,
    p: &Particle,
    f: &Fluid,
    h: f32,
    grid: &Vec<Vec<Entity>>,
    data: &HashMap<Entity, Vec2>,
) -> Particle {
    let mut x = p.position + p.velocity * h;
    let mut v = p.velocity;

    for other_id in &grid[f.grid_idx(&p)] {
        if other_id != &id {
            match data.get(other_id) {
                Some(other_x) => {
                    let mag = smooth(other_x.distance(p.position), f.radius);
                    let dir = (p.position - *other_x)
                        .try_normalize()
                        .unwrap_or(Vec2::ZERO);
                    v += mag * dir;
                }
                None => {}
            }
        }
    }

    for i in 0..2 {
        (x[i], v[i]) = constrain_1d(x[i], v[i], f.bounds_min[i], f.bounds_max[i]);
    }

    Particle {
        position: x,
        velocity: v,
    }
}

fn constrain_1d(x: f32, v: f32, lower: f32, upper: f32) -> (f32, f32) {
    if x < lower {
        (lower, -v)
    } else if x > upper {
        (upper, -v)
    } else {
        (x, v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::Vec2;

    #[test]
    fn test_constrain_1d() {
        assert_eq!(constrain_1d(-3.0, -1.0, -2.0, 2.0), (-2.0, 1.0));
        assert_eq!(constrain_1d(3.0, -1.0, -2.0, 2.0), (2.0, 1.0));
        assert_eq!(constrain_1d(0.0, -1.0, -2.0, 2.0), (0.0, -1.0));
    }

    /// position, velocity before (4 flaots),
    /// then position, velocity after (4 floats),
    fn test_update_particle_helper(ps: &[f32; 8], f: &Fluid, h: f32) {
        let before = Particle {
            position: Vec2::new(ps[0], ps[1]),
            velocity: Vec2::new(ps[2], ps[3]),
        };

        let after = Particle {
            position: Vec2::new(ps[4], ps[5]),
            velocity: Vec2::new(ps[6], ps[7]),
        };

        let grid: Vec<Vec<Entity>> = vec![];
        let map: HashMap<Entity, Vec2> = HashMap::new();

        assert_eq!(
            update_particle(Entity::from_raw(0), &before, &f, h, &grid, &map),
            after
        );
    }

    #[test]
    fn test_update_particle() {
        let fluid = Fluid {
            n: 0,              // dummy
            radius: 0.0,       // dummy
            speed: 0.0,        // dummy
            grid_dims: [0, 0], // dummy

            bounds_min: [-500.0, -300.0],
            bounds_max: [500.0, 300.0],
        };

        let no_collision = [0.0, 0.0, 2.0, 2.0, 0.2, 0.2, 2.0, 2.0];
        test_update_particle_helper(&no_collision, &fluid, 0.1);

        let collision = [-490.0, 290.0, -200.0, 200.0, -500.0, 300.0, 200.0, -200.0];
        test_update_particle_helper(&collision, &fluid, 0.1);
    }
}
