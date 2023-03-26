use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1280., 720.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_startup_system(setup)
        .add_system(cast_and_move)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4)),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    const GROUND_SIZE: f32 = 1000.; // Change to 10 to reduce jittering

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(GROUND_SIZE, 0.2, GROUND_SIZE))),
            material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
            transform: Transform::from_xyz(0., -0.1, 0.),
            ..default()
        },
        Collider::cuboid(GROUND_SIZE / 2., 0.1, GROUND_SIZE / 2.),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1., 1., 1.))),
            material: materials.add(Color::rgb(0.5, 0.9, 0.2).into()),
            transform: Transform::from_xyz(0., 0.5, 0.),
            ..default()
        },
        Collider::cuboid(0.5, 0.5, 0.5),
        RigidBody::Dynamic,
        CastPositionX::default(),
    ));
}

#[derive(Debug, Component, Default)]
struct CastPositionX(f32);

fn cast_and_move(
    mut q_player: Query<(&mut Transform, &mut CastPositionX)>,
    physics: Res<RapierContext>,
) {
    let (mut transform, mut position_x) = q_player.single_mut();

    position_x.0 += 0.001;

    if let Some((_entity, hit)) = physics.cast_shape(
        Vec3::new(0., 5., 0.),
        Quat::default(),
        Vec3::new(position_x.0, -2., 0.),
        &Collider::ball(0.5), // &Collider::cuboid(0.5, 0.5, 0.5) does not jitter
        10.,
        QueryFilter::exclude_dynamic(),
    ) {
        transform.translation.x = hit.witness1.x;
    }
}
