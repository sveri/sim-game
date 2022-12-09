
use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use crate::{components::{NameComponent}};

pub fn setup(
    mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    create_spheres(20, &mut commands, &mut meshes, &mut images, &mut materials, 0);
    create_spheres(20, &mut commands, &mut meshes, &mut images, &mut materials, 1);
    create_spheres(20, &mut commands, &mut meshes, &mut images, &mut materials, 2);
    create_spheres(20, &mut commands, &mut meshes, &mut images, &mut materials, 3);
    create_spheres(20, &mut commands, &mut meshes, &mut images, &mut materials, 4);
    create_spheres(20, &mut commands, &mut meshes, &mut images, &mut materials, 5);

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
