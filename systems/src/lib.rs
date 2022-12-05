mod utilities;

use bevy::{prelude::*, sprite::collide_aabb};
// use components::Name as CompName;
use components::*;
use rand::{thread_rng, Rng};

use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController},
    LookTransform, LookTransformBundle, Smoother,
};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // commands.spawn(Camera3dBundle {
    //     transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // }).insert(Name::new("Camera3d"));

    let eye = Vec3::new(-2.0, 2.5, 5.0);
    let target = Vec3::default();

    commands
        .spawn(Camera3dBundle {
            // camera_render_graph: CameraRenderGraph::new(bevy_strolle::graph::NAME),
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(OrbitCameraBundle::new(
            {
                let mut controller = OrbitCameraController::default();

                controller.mouse_rotate_sensitivity = Vec2::ONE * 0.2;
                controller.mouse_translate_sensitivity = Vec2::ONE * 0.5;

                controller
            },
            Vec3::new(-20.0, 10.0, 20.0),
            Vec3::ZERO,
        ));

    // commands
    //     .spawn(LookTransformBundle {
    //         // transform: LookTransform::new(eye, target),
    //         transform: LookTransform::new(eye, target),
    //         smoother: Smoother::new(0.9), // Value between 0.0 and 1.0, higher is smoother.
    //     })
    //     .insert(Camera3dBundle::default());

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.9 })),
        material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    commands.spawn((Galaxy, NameComponent("GalaxyOne".to_string())));
    commands.spawn((Galaxy, NameComponent("GalaxyTwo".to_string())));
}

#[no_mangle]
pub fn greet_galaxy(query: Query<&NameComponent, With<Galaxy>>) {
    for name in query.iter() {
        println!("hello {}", name.0);
    }
}

#[no_mangle]
pub fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
    time: Res<Time>,
) {
    const SPEED: f32 = 300.0;

    let (ship, mut transform) = query.single_mut();

    let mut rotation_factor = 0.0;
    let mut movement_factor = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        rotation_factor += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        rotation_factor -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        movement_factor += 1.0;
    }

    // update the ship rotation around the Z axis (perpendicular to the 2D plane of the screen)
    transform.rotate_z(rotation_factor * ship.rotation_speed * time.delta_seconds());

    // get the ship's forward vector by applying the current rotation to the ships initial facing vector
    let movement_direction = transform.rotation * Vec3::Y;
    // get the distance the ship will move based on direction, the ship's movement speed and delta time
    let movement_distance = movement_factor * SPEED * time.delta_seconds();
    // create the change in translation using the new movement direction and distance
    let translation_delta = movement_direction * movement_distance;
    // update the ship translation with our new translation delta
    transform.translation += translation_delta;

    // bound the ship within the invisible level bounds
    let extents = Vec3::from((BOUNDS / 2.0, 0.0));
    transform.translation = transform.translation.min(extents).max(-extents);
}

#[no_mangle]
pub fn player_shooting_system(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<&Transform, With<Player>>,
) {
    const SIZE: f32 = 10.0;

    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Ok(tfm) = query.get_single() {
            commands
                .spawn_bundle(SpriteBundle {
                    transform: *tfm,
                    sprite: Sprite {
                        color: Color::rgb(0.9, 0.8, 0.0),
                        custom_size: Some(Vec2::new(SIZE, SIZE)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Bullet);
        }
    }
}

#[no_mangle]
pub fn bullet_movement_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<Bullet>>,
    cam: Query<&Camera>,
    time: Res<Time>,
) {
    let screen_size = cam.single().logical_viewport_size().unwrap() * 0.5;
    let speed = 800.0;
    for (entity, mut tfm) in &mut query {
        let x = tfm
            .rotation
            .mul_vec3(Vec3::new(0.0, speed * time.delta_seconds(), 0.0));
        tfm.translation += x;

        if utilities::is_outside_bounds(
            tfm.translation.truncate(),
            (
                (-screen_size.x),
                screen_size.y,
                screen_size.x,
                (-screen_size.y),
            ),
        ) {
            log::info!("pufff");
            commands.entity(entity).despawn();
        }
    }
}

#[no_mangle]
pub fn bullet_hit_system(
    mut commands: Commands,
    bullet_query: Query<&Transform, With<Bullet>>,
    ship_query: Query<(Entity, &Transform), With<OtherShip>>,
) {
    for bullet_tfm in bullet_query.iter() {
        for (entity, ship_tfm) in ship_query.iter() {
            if collide_aabb::collide(
                bullet_tfm.translation,
                Vec2::new(10.0, 10.0),
                ship_tfm.translation,
                Vec2::new(30.0, 30.0),
            )
            .is_some()
            {
                log::info!("BUUMMMM");
                commands.entity(entity).despawn();
            }
        }
    }
}

#[no_mangle]
pub fn spawn_other_ships(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    others: Query<(Entity, &Transform), With<OtherShip>>,
    cam: Query<&Camera>,
) {
    const MARGIN: f32 = 30.0;
    const MIN_SHIP_COUNT: usize = 10;

    let screen_size = cam.single().logical_viewport_size().unwrap() * 0.5;
    let mut other_ships_count = 0;

    for (entity, tfm) in others.iter() {
        if utilities::is_outside_bounds(
            tfm.translation.truncate(),
            (
                (-screen_size.x) - MARGIN,
                screen_size.y + MARGIN,
                screen_size.x + MARGIN,
                (-screen_size.y) - MARGIN,
            ),
        ) {
            commands.entity(entity).despawn();
        } else {
            other_ships_count += 1;
        }
    }

    if other_ships_count < MIN_SHIP_COUNT {
        let x = if thread_rng().gen::<bool>() {
            thread_rng().gen_range(((-screen_size.x) - MARGIN)..(-screen_size.x))
        } else {
            thread_rng().gen_range(screen_size.x..(screen_size.x + MARGIN))
        };
        let y = if thread_rng().gen::<bool>() {
            thread_rng().gen_range(((-screen_size.y) - MARGIN)..(-screen_size.y))
        } else {
            thread_rng().gen_range(screen_size.y..(screen_size.y + MARGIN))
        };
        let dir = thread_rng().gen_range(0.0f32..360.0f32);
        let mut transform = Transform::from_xyz(x, y, 0.0);
        transform.rotate_z(dir.to_radians());

        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("textures/simplespace/enemy_A.png"),
                transform,
                ..default()
            })
            .insert(OtherShip);
    }
}

#[no_mangle]
pub fn move_other_ships(time: Res<Time>, mut query: Query<&mut Transform, With<OtherShip>>) {
    const SPEED: f32 = 100.0;
    for mut tfm in &mut query {
        let x = tfm
            .rotation
            .mul_vec3(Vec3::new(0.0, SPEED * time.delta_seconds(), 0.0));

        tfm.translation += x;
    }
}
