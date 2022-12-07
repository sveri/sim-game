mod utilities;

use std::f32::consts::PI;

use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    sprite::collide_aabb,
};
// use components::Name as CompName;
use components::*;
use rand::{thread_rng, Rng};

use smooth_bevy_cameras::controllers::orbit::{self, OrbitCameraBundle, OrbitCameraController};


pub fn pan_orbit_camera(
    windows: Res<Windows>, mut ev_emotion: EventReader<MouseMotion>, mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>, mut query: Query<(&mut PanOrbitCamera, &mut Transform, &Projection)>,
) {
    let orbit_button = MouseButton::Right;
    let pan_button = MouseButton::Middle;
    let mut pan = Vec2::ZERO;
    let mut rotation_move = Vec2::ZERO;
    let scroll: f32 = 0.0;
    let orbit_button_changed = false;

    if input_mouse.pressed(orbit_button) {
        print!("rigggght");
        for ev in ev_emotion.iter() {
            rotation_move += ev.delta;
        }
    } else if input_mouse.pressed(pan_button) {
        for ev in ev_emotion.iter() {
            pan += ev.delta;
        }
    }

    for (mut pan_orbit, mut transform, projection) in query.iter_mut() {
        if orbit_button_changed {
            let up = transform.rotation * Vec3::Y;
            pan_orbit.upside_down = up.y <= 0.0;
        }

        let mut any = false;
        if rotation_move.length_squared() > 0.0 {
            any = true;
            let window = get_primary_window_size(&windows);
            if let Projection::Perspective(projection) = projection {
                pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
            }

            let right = transform.rotation * Vec3::X * -pan.x;
            let up = transform.rotation * Vec3::Y * pan.y;
            let translation = (right + up) * pan_orbit.radius;
            pan_orbit.focus += translation;
        } else if scroll.abs() > 0.0 {
            any = true;
            pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
            pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
        }

        if any {
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation = pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
        }
    }
}


fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    Vec2::new(window.width(), window.height())
}

pub fn setup(
    mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let translation = Vec3::new(-2.0, 2.5, 5.0);
    let radius = translation.length();

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera {
            radius,
            ..Default::default()
        },
    ));
    // commands
    //     .spawn(Camera3dBundle {
    //         transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    //         ..default()
    //     })
    //     .insert(OrbitCameraBundle::new(
    //         OrbitCameraController {
    //             mouse_rotate_sensitivity: Vec2::ONE,
    //             mouse_translate_sensitivity: Vec2::ONE * 0.5,
    //             ..default()
    //         },
    //         Vec3::new(-20.0, 10.0, 20.0),
    //         Vec3::ZERO,
    //     ))
    //     .insert(Name::new("Camera3d"));

    let mut i = 0;

    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    while i < 10 {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius: 2.0,
                    sectors: 100,
                    stacks: 100,
                })),
                material: debug_material.clone(),
                transform: Transform::from_xyz(i as f32 * 10.0, 0.5, 0.0).with_rotation(Quat::from_rotation_x(-PI / 4.)),
                ..default()
            },
            NameComponent("Circle 2".to_string()),
        ));
        i += 1;
    }

    commands.spawn((Galaxy, NameComponent("GalaxyOne".to_string())));
    commands.spawn((Galaxy, NameComponent("GalaxyTwo".to_string())));
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255, 198, 255, 102, 198, 255, 255,
        121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
}

#[no_mangle]
pub fn greet_galaxy(query: Query<&NameComponent, With<Galaxy>>) {
    for name in query.iter() {
        println!("hello {}", name.0);
    }
}

#[no_mangle]
pub fn player_movement_system(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&Player, &mut Transform)>, time: Res<Time>) {
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
    mut commands: Commands, keyboard_input: Res<Input<KeyCode>>, query: Query<&Transform, With<Player>>,
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
    mut commands: Commands, mut query: Query<(Entity, &mut Transform), With<Bullet>>, cam: Query<&Camera>, time: Res<Time>,
) {
    let screen_size = cam.single().logical_viewport_size().unwrap() * 0.5;
    let speed = 800.0;
    for (entity, mut tfm) in &mut query {
        let x = tfm.rotation.mul_vec3(Vec3::new(0.0, speed * time.delta_seconds(), 0.0));
        tfm.translation += x;

        if utilities::is_outside_bounds(
            tfm.translation.truncate(),
            ((-screen_size.x), screen_size.y, screen_size.x, (-screen_size.y)),
        ) {
            log::info!("pufff");
            commands.entity(entity).despawn();
        }
    }
}

#[no_mangle]
pub fn bullet_hit_system(
    mut commands: Commands, bullet_query: Query<&Transform, With<Bullet>>,
    ship_query: Query<(Entity, &Transform), With<OtherShip>>,
) {
    for bullet_tfm in bullet_query.iter() {
        for (entity, ship_tfm) in ship_query.iter() {
            if collide_aabb::collide(bullet_tfm.translation, Vec2::new(10.0, 10.0), ship_tfm.translation, Vec2::new(30.0, 30.0))
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
    mut commands: Commands, asset_server: Res<AssetServer>, others: Query<(Entity, &Transform), With<OtherShip>>,
    cam: Query<&Camera>,
) {
    const MARGIN: f32 = 30.0;
    const MIN_SHIP_COUNT: usize = 10;

    let screen_size = cam.single().logical_viewport_size().unwrap() * 0.5;
    let mut other_ships_count = 0;

    for (entity, tfm) in others.iter() {
        if utilities::is_outside_bounds(
            tfm.translation.truncate(),
            ((-screen_size.x) - MARGIN, screen_size.y + MARGIN, screen_size.x + MARGIN, (-screen_size.y) - MARGIN),
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
        let x = tfm.rotation.mul_vec3(Vec3::new(0.0, SPEED * time.delta_seconds(), 0.0));

        tfm.translation += x;
    }
}
