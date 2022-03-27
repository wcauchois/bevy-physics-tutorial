use bevy::prelude::*;
use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin};
use bevy_rapier3d::{
    physics::{
        ColliderBundle, NoUserData, RapierPhysicsPlugin, RigidBodyBundle, RigidBodyPositionSync,
    },
    prelude::{ColliderMaterial, ColliderShape},
};

#[derive(Bundle)]
struct CoolCubeBundle {
    #[bundle]
    pbr_bundle: PbrBundle,
    #[bundle]
    rigid_body_bundle: RigidBodyBundle,
    #[bundle]
    collider_bundle: ColliderBundle,
    rigid_body_position_sync: RigidBodyPositionSync,
}

impl CoolCubeBundle {
    fn new(
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        Self {
            pbr_bundle: PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                ..Default::default()
            },
            rigid_body_bundle: RigidBodyBundle {
                position: Vec3::new(0.0, 10.0, 0.0).into(),
                ..Default::default()
            },
            collider_bundle: ColliderBundle {
                shape: ColliderShape::cuboid(0.5, 0.5, 0.5).into(),
                material: ColliderMaterial {
                    restitution: 0.7,
                    ..Default::default()
                }
                .into(),
                ..Default::default()
            },
            rigid_body_position_sync: RigidBodyPositionSync::Discrete,
        }
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Plane (ground)
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(100.0, 0.001, 100.0).into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete);
    // Cube
    commands.spawn_bundle(CoolCubeBundle::new(meshes, materials));
    // Light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // Camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(FlyCam);
}

fn move_light(time: Res<Time>, mut query: Query<&mut Transform, With<PointLight>>) {
    let factor = (time.seconds_since_startup() as f32) % 20.0 - 10.0;
    let mut transform = query.single_mut();
    transform.translation = Vec3::new(4.0, 8.0, 4.0 + factor);
}

// fn spawn_cubes_on_click(buttons: Res<Input<MouseButton>>, fly_cam_query: Query<) {
//     if buttons.just_pressed(MouseButton::Left) {}
// }

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(NoCameraPlayerPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(MovementSettings {
            sensitivity: 0.00012, // default: 0.00012
            speed: 12.0,          // default: 12.0
        })
        .add_startup_system(setup)
        .add_system(move_light)
        .run()
}
