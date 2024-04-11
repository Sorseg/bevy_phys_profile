use bevy::prelude::{
    debug, App, Commands, Component, GlobalTransform, In, IntoSystem, Query, Startup, Transform,
    TransformBundle, Update, Vec3, With,
};
use bevy_xpbd_3d::prelude::{Collider, ExternalForce, LockedAxes, PhysicsPlugins, RigidBody};

#[derive(Component)]
struct Player;

pub fn setup_xpbd(app: &mut App, n: usize) {
    app.add_plugins(PhysicsPlugins::default())
        .add_systems(Startup, ((move || n).pipe(spawn_colliders), spawn_player))
        .add_systems(Update, (move || n as f32).pipe(reset_player));
}

fn spawn_colliders(In(n): In<usize>, mut commands: Commands) {
    for x in 0..n {
        for z in 0..n {
            commands.spawn((
                RigidBody::Static,
                TransformBundle::from_transform(Transform::from_xyz(
                    x as f32 - n as f32 / 2.0,
                    0.0,
                    z as f32 - n as f32 / 2.0,
                )),
                Collider::cuboid(1.0, 1.0, 1.0),
            ));
        }
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)),
        Collider::capsule(0.5, 0.5),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        // disturbing the system a little bit
        ExternalForce::new(Vec3 {
            x: 100.0,
            y: 0.0,
            z: 0.0,
        })
        .with_persistence(true),
    ));
}

fn reset_player(
    In(max_travel): In<f32>,
    mut q: Query<(&GlobalTransform, &mut Transform), With<Player>>,
) {
    let (glob, mut tr) = q.single_mut();
    let glob = glob.translation();
    debug!(player_pos=?glob);
    if glob.to_array().iter().any(|c| *c > max_travel / 2.0) {
        *tr = Transform::from_xyz(0.0, 4.0, 0.0);
    }
}
