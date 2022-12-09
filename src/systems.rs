use std::f32::consts::PI;

use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use crate::components::{NameComponent, PanOrbitCamera};

// pub fn pan_orbit_camera(
//     windows: Res<Windows>,
//     mut ev_motion: EventReader<MouseMotion>,
//     mut ev_scroll: EventReader<MouseWheel>,
//     input_mouse: Res<Input<MouseButton>>,
//     mut query: Query<(&mut PanOrbitCamera, &mut Transform, &PerspectiveProjection)>,
// ) {
//     // change input mapping for orbit and panning here
//     let orbit_button = MouseButton::Right;
//     let pan_button = MouseButton::Middle;

//     let mut pan = Vec2::ZERO;
//     let mut rotation_move = Vec2::ZERO;
//     let mut scroll = 0.0;
//     let mut orbit_button_changed = false;

//     if input_mouse.pressed(orbit_button) {
//         for ev in ev_motion.iter() {
//             rotation_move += ev.delta;
//         }
//     } else if input_mouse.pressed(pan_button) {
//         // Pan only if we're not rotating at the moment
//         for ev in ev_motion.iter() {
//             pan += ev.delta;
//         }
//     }
//     for ev in ev_scroll.iter() {
//         scroll += ev.y;
//     }
//     if input_mouse.just_released(orbit_button) || input_mouse.just_pressed(orbit_button) {
//         orbit_button_changed = true;
//     }

//     for (mut pan_orbit, mut transform, projection) in query.iter_mut() {
//         if orbit_button_changed {
//             // only check for upside down when orbiting started or ended this frame
//             // if the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct
//             let up = transform.rotation * Vec3::Y;
//             pan_orbit.upside_down = up.y <= 0.0;
//         }

//         let mut any = false;
//         if rotation_move.length_squared() > 0.0 {
//             any = true;
//             let window = get_primary_window_size(&windows);
//             let delta_x = {
//                 let delta = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
//                 if pan_orbit.upside_down { -delta } else { delta }
//             };
//             let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
//             let yaw = Quat::from_rotation_y(-delta_x);
//             let pitch = Quat::from_rotation_x(-delta_y);
//             transform.rotation = yaw * transform.rotation; // rotate around global y axis
//             transform.rotation = transform.rotation * pitch; // rotate around local x axis
//         } else if pan.length_squared() > 0.0 {
//             any = true;
//             // make panning distance independent of resolution and FOV,
//             let window = get_primary_window_size(&windows);
//             pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
//             // translate by local axes
//             let right = transform.rotation * Vec3::X * -pan.x;
//             let up = transform.rotation * Vec3::Y * pan.y;
//             // make panning proportional to distance away from focus point
//             let translation = (right + up) * pan_orbit.radius;
//             pan_orbit.focus += translation;
//         } else if scroll.abs() > 0.0 {
//             any = true;
//             pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
//             // dont allow zoom to reach zero or you get stuck
//             pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
//         }

//         if any {
//             // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
//             // parent = x and y rotation
//             // child = z-offset
//             let rot_matrix = Mat3::from_quat(transform.rotation);
//             transform.translation = pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
//         }
//     }
// }

/// Spawn a camera like this
// fn spawn_camera(mut commands: Commands) {
//     let translation = Vec3::new(-0.0, -1.0, -1.0);
//     let radius = translation.length();

//     commands.spawn_bundle(PerspectiveCameraBundle {
//         transform: Transform::from_translation(translation)
//             .looking_at(Vec3::ZERO, Vec3::Y),
//         ..Default::default()
//     }).insert(PanOrbitCamera {
//         radius,
//         ..Default::default()
//     });
// }

pub fn pan_orbit_camera(
    windows: Res<Windows>, mut ev_motion: EventReader<MouseMotion>, mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>, mut query: Query<(&mut PanOrbitCamera, &mut Transform, &Projection)>,
) {
    // change input mapping for orbit and panning here
    let orbit_button = MouseButton::Right;
    let pan_button = MouseButton::Middle;

    let mut pan = Vec2::ZERO;
    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;
    let mut orbit_button_changed = false;

    if input_mouse.pressed(orbit_button) {
        for ev in ev_motion.iter() {
            rotation_move += ev.delta;
        }
    } else if input_mouse.pressed(pan_button) {
        // Pan only if we're not rotating at the moment
        for ev in ev_motion.iter() {
            pan += ev.delta;
        }
    }
    for ev in ev_scroll.iter() {
        scroll += ev.y;
    }
    if input_mouse.just_released(orbit_button) || input_mouse.just_pressed(orbit_button) {
        orbit_button_changed = true;
    }

    for (mut pan_orbit, mut transform, projection) in query.iter_mut() {
        if orbit_button_changed {
            // only check for upside down when orbiting started or ended this frame
            // if the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct
            let up = transform.rotation * Vec3::Y;
            pan_orbit.upside_down = up.y <= 0.0;
        }

        let mut any = false;
        if rotation_move.length_squared() > 0.0 {
            any = true;
            let window = get_primary_window_size(&windows);
            let delta_x = {
                let delta = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
                if pan_orbit.upside_down {
                    -delta
                } else {
                    delta
                }
            };
            let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation; // rotate around global y axis
            transform.rotation = transform.rotation * pitch; // rotate around local x axis
        } else if pan.length_squared() > 0.0 {
            any = true;
            // make panning distance independent of resolution and FOV,
            let window = get_primary_window_size(&windows);
            if let Projection::Perspective(projection) = projection {
                pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
            }
            // translate by local axes
            let right = transform.rotation * Vec3::X * -pan.x;
            let up = transform.rotation * Vec3::Y * pan.y;
            // make panning proportional to distance away from focus point
            let translation = (right + up) * pan_orbit.radius;
            pan_orbit.focus += translation;
        } else if scroll.abs() > 0.0 {
            any = true;
            pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
            // dont allow zoom to reach zero or you get stuck
            pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
        }

        if any {
            // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
            // parent = x and y rotation
            // child = z-offset
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation = pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
        }
    }
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    Vec2::new(window.width(), window.height())
}

// pub struct PanOrbitCameraPlugin;

// impl Plugin for PanOrbitCameraPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_system(pan_orbit_camera);
//         //.add_system_to_stage(CoreStage::PostUpdate, pan_orbit_camera);
//     }
// }

pub fn setup(
    mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let translation = Vec3::new(-2.0, 2.5, 5.0);
    let radius = translation.length();

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        PanOrbitCamera {
            radius,
            ..Default::default()
        },
    ));

    // let b_mesh = &mut *meshes;

    create_spheres(20, &mut commands, &mut meshes, &mut images, &mut materials, 0);
    create_spheres(20, &mut commands, &mut meshes, &mut images, &mut materials, 1);
    create_spheres(20, &mut commands, &mut meshes, &mut images, &mut materials, 2);
    create_spheres(20, &mut commands, &mut meshes, &mut images, &mut materials, 3);
    create_spheres(20, &mut commands, &mut meshes, &mut images, &mut materials, 4);
    create_spheres(20, &mut commands, &mut meshes, &mut images, &mut materials, 5);
    // create_spheres(20, &commands, &meshes, &images, &materials, 2);

    fn create_spheres(
        count: i32, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, images: &mut ResMut<Assets<Image>>,
        materials: &mut ResMut<Assets<StandardMaterial>>, direction: u8,
    ) {
        let debug_material = materials.add(StandardMaterial {
            base_color_texture: Some(images.add(uv_debug_texture())),
            ..default()
        });

        let mut i = 0;

        while i < count {
            let trans = match direction {
                0 => Transform::from_xyz(i as f32 * 20.0, 0.5, 0.0),
                1 => Transform::from_xyz(0., i as f32 * 20.0, 0.0),
                2 => Transform::from_xyz(0., 0., i as f32 * 20.0),
                3 => Transform::from_xyz(-i as f32 * 20.0, 0.5, 0.0),
                4 => Transform::from_xyz(0., -i as f32 * 20.0, 0.0),
                5 => Transform::from_xyz(0., 0., -i as f32 * 20.0),
                6..=u8::MAX => Transform::from_xyz(i as f32 * 20.0, 0.5, 0.0),
            };
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::UVSphere {
                        radius: 10.0,
                        sectors: 100,
                        stacks: 100,
                    })),
                    material: debug_material.clone(),
                    transform: trans,
                    ..default()
                },
                NameComponent("Circle 2".to_string()),
            ));
            i += 1;
        }
    }

    // let mut i = 0;

    // while i < 100 {
    //     commands.spawn((
    //         PbrBundle {
    //             mesh: meshes.add(Mesh::from(shape::UVSphere {
    //                 radius: 10.0,
    //                 sectors: 100,
    //                 stacks: 100,
    //             })),
    //             material: debug_material.clone(),
    //             transform: Transform::from_xyz(i as f32 * 20.0, 0.5, 0.0).with_rotation(Quat::from_rotation_x(-PI / 4.)),
    //             ..default()
    //         },
    //         NameComponent("Circle 2".to_string()),
    //     ));
    //     i += 1;
    // }

    // i = 0;

    // while i < 100 {
    //     commands.spawn((
    //         PbrBundle {
    //             mesh: meshes.add(Mesh::from(shape::UVSphere {
    //                 radius: 3.0,
    //                 sectors: 100,
    //                 stacks: 100,
    //             })),
    //             material: debug_material.clone(),
    //             transform: Transform::from_xyz(0.0, i as f32 * 10.0, 0.0).with_rotation(Quat::from_rotation_x(-PI / 4.)),
    //             ..default()
    //         },
    //         NameComponent("Circle 2".to_string()),
    //     ));
    //     i += 1;
    // }
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
