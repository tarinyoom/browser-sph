use bevy::prelude::*;
use itertools::izip;

use super::{GameComponent, GameResource};

use crate::model::{step, Globals, Grid, Particle};

pub fn update_grid(
    particles: Query<(Entity, &GameComponent<Particle>)>,
    mut grid: ResMut<GameResource<Grid<Entity>>>,
) {
    let particle_iter = particles.iter().map(|(id, comp)| (id, comp.val.clone()));
    grid.val.fill(particle_iter);
}

pub fn update_particles(
    mut particles: Query<(Entity, &mut GameComponent<Particle>)>,
    globals: Res<GameResource<Globals>>,
    time: Res<Time<Fixed>>,
    grid: Res<GameResource<Grid<Entity>>>,
) {
    let g = &globals.into_inner().val;
    for (e, mut p) in &mut particles {
        p.val = step(e, &p.val, (*time).delta_seconds_f64(), g, &grid.val);
    }
}

fn make_color(rho: f32, mean: f32, stdev: f32) -> Color {
    let normalized = ((rho - mean) / stdev) as f32;
    let r = (normalized * 0.25 + 0.5).min(1.0).max(0.0);
    let gb = (1.0 - r) * 0.7;
    return Color::rgb(r, gb, gb);
}

fn area(g: &Globals) -> f32 {
    izip!(&g.bounds_min, &g.bounds_max)
        .map(|(min, max)| max - min)
        .fold(1.0, |a, b| a * b) as f32
}

pub fn update_transforms(
    mut particles: Query<(
        &mut Transform,
        &Handle<ColorMaterial>,
        &mut GameComponent<Particle>,
    )>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    globals: Res<GameResource<Globals>>,
) {
    for (mut transform, color, p) in &mut particles {
        transform.translation = unwrap(&p.val);
        let color_mat = materials.get_mut(&*color).unwrap();
        let n = globals.val.n as f32;
        let mean = n / area(&globals.val);
        let stdev = f32::sqrt(n) / 40000.0;
        color_mat.color = make_color(p.val.density as f32, mean, stdev);
    }
}

fn unwrap(p: &Particle) -> Vec3 {
    let x = &p.position;
    Vec3::new(x[0] as f32, x[1] as f32, 0.0)
}
