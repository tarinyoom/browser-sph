use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use itertools::izip;
use rand::prelude::ThreadRng;

use super::{GameComponent, GameResource, ParticleBundle};

use crate::model::{generate_particle, Globals, Grid};

pub fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let globals = Globals::default();
    let mut rng: ThreadRng = rand::thread_rng();
    let p_vz_sz: f32 = 3.0;
    let bg_sz: Vec<f32> = izip!(&globals.bounds_min, &globals.bounds_max)
        .map(|(min, max)| (max - min) as f32 + 2.0 * p_vz_sz)
        .collect();
    let bg_center: Vec<f32> = izip!(&globals.bounds_min, &globals.bounds_max)
        .map(|(min, max)| (min + max) as f32 / 2.0)
        .collect();
    let bg = MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(bg_sz[0], bg_sz[1])).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::BLACK)),
        transform: Transform {
            translation: Vec3::new(bg_center[0], bg_center[1], -10.0),
            rotation: Quat::from_xyzw(0.0, 0.0, 0.0, 1.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
        },
        ..default()
    };
    commands.spawn(bg);

    for _ in 0..globals.n {
        let p: ParticleBundle = ParticleBundle {
            mesh: meshes.add(shape::Circle::new(p_vz_sz).into()).into(),
            material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            inherited_visibility: Default::default(),
            view_visibility: Default::default(),
            particle: GameComponent {
                val: generate_particle(&mut rng, &globals),
            },
        };
        commands.spawn(p);
    }
    commands.insert_resource(GameResource {
        val: Grid::<Entity>::new(&globals),
    });
    commands.insert_resource(GameResource { val: globals });
    commands.spawn(Camera2dBundle::default());
}
