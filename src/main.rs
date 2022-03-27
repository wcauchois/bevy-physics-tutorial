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

#[derive(Component)]
struct CursorThing;

impl CoolCubeBundle {
    fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        position: Vec3,
    ) -> Self {
        Self {
            pbr_bundle: PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                ..Default::default()
            },
            rigid_body_bundle: RigidBodyBundle {
                position: position.into(),
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
            mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(100.0, 0.001, 100.0).into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete);
    // Cube
    commands.spawn_bundle(CoolCubeBundle::new(
        &mut meshes,
        &mut materials,
        Vec3::new(0.0, 10.0, 0.0),
    ));
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
    // Cursor thingy
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 0.2,
                subdivisions: 32,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgba(0.9, 0.9, 0.0, 0.5),
                alpha_mode: AlphaMode::Blend,
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert(CursorThing);
}

fn move_light(time: Res<Time>, mut query: Query<&mut Transform, With<PointLight>>) {
    let factor = (time.seconds_since_startup() as f32) % 20.0 - 10.0;
    let mut transform = query.single_mut();
    transform.translation = Vec3::new(4.0, 8.0, 4.0 + factor);
}

fn spawn_position_for_transform(transform: &Transform) -> Vec3 {
    transform.translation + (transform.rotation.mul_vec3(Vec3::Z) * -6.0)
}

fn move_cursor_thing(
    mut q: QuerySet<(
        QueryState<&mut Transform, With<CursorThing>>,
        QueryState<&Transform, With<FlyCam>>,
    )>,
) {
    let pos = spawn_position_for_transform(q.q1().single());
    q.q0().single_mut().translation = pos;
}

fn spawn_cubes_on_click(
    buttons: Res<Input<MouseButton>>,
    camera_query: Query<&Transform, With<FlyCam>>,

    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let camera_transform = camera_query.single();
    if buttons.just_pressed(MouseButton::Left) {
        let pos = spawn_position_for_transform(camera_transform);
        commands.spawn_bundle(CoolCubeBundle::new(&mut meshes, &mut materials, pos));
    }
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(bevy_atmosphere::AtmosphereMat::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(NoCameraPlayerPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(bevy_atmosphere::AtmospherePlugin {
            dynamic: false, // Set to false since we aren't changing the sky's appearance
            sky_radius: 10.0,
        })
        .insert_resource(MovementSettings {
            sensitivity: 0.00012, // default: 0.00012
            speed: 12.0,          // default: 12.0
        })
        .add_startup_system(setup)
        .add_system(move_light)
        .add_system(spawn_cubes_on_click)
        .add_system(move_cursor_thing)
        .run()
}
