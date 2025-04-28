mod types;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use rand::{rngs::ThreadRng, Rng};

use crate::types::*;

const MEMBER_COUNT: usize = 256;
const PIXEL_THICKNESS: f32 = 3.0;
const DISTRIBUTION_SIZE: f32 = 640.0;

const MARGIN: f32 = DISTRIBUTION_SIZE / 2.0;
const MARGIN_WIDTH: f32 = DISTRIBUTION_SIZE / 32.0;

const INFLUENCE_RANGE: f32 = DISTRIBUTION_SIZE / 8.0;
const VISCOSITY_FACTOR: f32 = 0.999975;

const ATTRACTION: f32 = 0.2;
const REPULSION: f32 = -ATTRACTION / 5.0;
const INDIFFERENCE: f32 = ATTRACTION / 10.0;
const CONGREGATION: f32 = -INDIFFERENCE * 5.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, system_setup)
        .add_systems(Update, (system_animate, system_copy))
        .run();
}

fn system_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let mut forces = Forces::new();
    forces.set_force(&Race::Red, &Race::Red, CONGREGATION);
    forces.set_force(&Race::Red, &Race::Green, ATTRACTION);
    forces.set_force(&Race::Red, &Race::Blue, INDIFFERENCE);
    forces.set_force(&Race::Red, &Race::Yellow, INDIFFERENCE);
    forces.set_force(&Race::Red, &Race::Violet, REPULSION);

    forces.set_force(&Race::Green, &Race::Red, REPULSION);
    forces.set_force(&Race::Green, &Race::Green, CONGREGATION);
    forces.set_force(&Race::Green, &Race::Blue, ATTRACTION);
    forces.set_force(&Race::Green, &Race::Yellow, INDIFFERENCE);
    forces.set_force(&Race::Green, &Race::Violet, INDIFFERENCE);

    forces.set_force(&Race::Blue, &Race::Red, INDIFFERENCE);
    forces.set_force(&Race::Blue, &Race::Green, REPULSION);
    forces.set_force(&Race::Blue, &Race::Blue, CONGREGATION);
    forces.set_force(&Race::Blue, &Race::Yellow, ATTRACTION);
    forces.set_force(&Race::Blue, &Race::Violet, INDIFFERENCE);

    forces.set_force(&Race::Yellow, &Race::Red, INDIFFERENCE);
    forces.set_force(&Race::Yellow, &Race::Green, INDIFFERENCE);
    forces.set_force(&Race::Yellow, &Race::Blue, REPULSION);
    forces.set_force(&Race::Yellow, &Race::Yellow, CONGREGATION);
    forces.set_force(&Race::Yellow, &Race::Violet, ATTRACTION);

    forces.set_force(&Race::Violet, &Race::Red, ATTRACTION);
    forces.set_force(&Race::Violet, &Race::Green, INDIFFERENCE);
    forces.set_force(&Race::Violet, &Race::Blue, INDIFFERENCE);
    forces.set_force(&Race::Violet, &Race::Yellow, REPULSION);
    forces.set_force(&Race::Violet, &Race::Violet, CONGREGATION);

    commands.insert_resource(forces);
    generate_background(&mut commands, &mut meshes, &mut materials);
    generate_swarm(&mut commands, &mut meshes, &mut materials, MEMBER_COUNT);
}

fn system_animate(
    time: Res<Time>,
    forces: Res<Forces>,
    mut target_drones: Query<(&Race, &mut Position2D, &mut Velocity2D, &mut Transform)>,
) {
    let delta_t = time.delta_seconds();
    let delta_t_sqr = delta_t * delta_t;
    let mut combinations = target_drones.iter_combinations_mut();

    while let Some([drone1, drone2]) = combinations.fetch_next() {
        let (race_1, mut position_1, mut velocity_1, transform_1) = drone1;
        let (race_2, mut position_2, mut velocity_2, transform_2) = drone2;

        update_translation(
            position_1.as_mut(),
            velocity_1.as_mut(),
            transform_1.as_ref(),
            transform_2.as_ref(),
            forces.get_force(race_1, race_2),
            delta_t,
            delta_t_sqr,
        );
        update_translation(
            position_2.as_mut(),
            velocity_2.as_mut(),
            transform_2.as_ref(),
            transform_1.as_ref(),
            forces.get_force(race_2, race_1),
            delta_t,
            delta_t_sqr,
        );
    }
}

fn system_copy(mut target_drones: Query<(&Position2D, &mut Transform)>) {
    for drone in target_drones.iter_mut() {
        let (p, mut t) = drone;
        {
            t.translation.x = p.x;
            t.translation.y = p.y;
        }
    }
}

fn update_translation(
    target_position: &mut Position2D,
    target_velocity: &mut Velocity2D,
    target_transform: &Transform,
    reference: &Transform,
    force_factor: f32,
    delta_t: f32,
    delta_t_sqr: f32,
) {
    if delta_t > 0.0 {
        let mut force_x = 0.0;
        let mut force_y = 0.0;
        let mut margin_repulsion_x = 1.0;
        let mut margin_repulsion_y = 1.0;
        let dx = reference.translation.x - target_transform.translation.x;
        let dy = reference.translation.y - target_transform.translation.y;
        let dist = (dx * dx + dy * dy).sqrt();

        let margin_dx = MARGIN - target_position.x.abs();
        let margin_dy = MARGIN - target_position.y.abs();

        if margin_dx < MARGIN_WIDTH {
            margin_repulsion_x = 2.0 * force_factor / margin_dx;
        }
        if margin_dy < MARGIN_WIDTH {
            margin_repulsion_y = 2.0 * force_factor / margin_dy;
        }

        if dist < INFLUENCE_RANGE {
            let force = force_factor / dist;

            force_x = dx.signum() * force * margin_repulsion_x * dx.abs();
            force_y = dy.signum() * force * margin_repulsion_y * dy.abs();
        }

        target_position.x =
            target_transform.translation.x + target_velocity.x * delta_t + force_x * delta_t_sqr;
        target_position.y =
            target_transform.translation.y + target_velocity.y * delta_t + force_y * delta_t_sqr;

        let mut delta_x = target_position.x - target_transform.translation.x;
        let mut delta_y = target_position.y - target_transform.translation.y;

        if target_position.x.abs() > MARGIN {
            delta_x *= -1.0;
        }
        if target_position.y.abs() > MARGIN {
            delta_y *= -1.0;
        }

        target_velocity.x = VISCOSITY_FACTOR * delta_x / delta_t;
        target_velocity.y = VISCOSITY_FACTOR * delta_y / delta_t;
    }
}

fn generate_background(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let shape = Mesh2dHandle(meshes.add(Rectangle::new(DISTRIBUTION_SIZE, DISTRIBUTION_SIZE)));
    commands.spawn(MaterialMesh2dBundle {
        mesh: shape,
        material: materials.add(Color::rgb(0.125, 0.125, 0.125)),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}

fn generate_swarm(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    count: usize,
) {
    let mut rng = rand::rng();

    for _ in 0..count {
        for race in Race::all() {
            let (shape, position) = create_mesh(meshes, &mut rng);
            let z = 1.0 + Race::all().len() as f32 * rng.random::<f32>();
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: shape,
                    material: materials.add(Color::from(race.clone())),
                    transform: Transform::from_xyz(position.x, position.y, z),
                    ..default()
                },
                race,
                position,
                Velocity2D { x: 0.0, y: 0.0 },
            ));
        }
    }
}

fn create_mesh(
    meshes: &mut ResMut<Assets<Mesh>>,
    rng: &mut ThreadRng,
) -> (Mesh2dHandle, Position2D) {
    let shape = Mesh2dHandle(meshes.add(Circle::new(PIXEL_THICKNESS)));
    let x = DISTRIBUTION_SIZE * rng.random::<f32>() - MARGIN - PIXEL_THICKNESS;
    let y = DISTRIBUTION_SIZE * rng.random::<f32>() - MARGIN - PIXEL_THICKNESS;

    (shape, Position2D { x, y })
}
