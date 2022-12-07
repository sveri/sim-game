use bevy::prelude::*;

pub const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);

#[derive(Component)]
pub struct PanOrbitCamera {
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
        }
    }
}

#[derive(Component)]
pub struct OtherShip;

#[derive(Component)]
pub struct Bullet;

/// player component
#[derive(Component)]
pub struct Player {
    // pub movement_speed: f32,
    pub velocity: Vec3,
    pub rotation_speed: f32,
    pub shooting_timer: Option<Timer>,
}

#[derive(Component)]
pub struct NameComponent(pub String);

#[derive(Component)]
pub struct Galaxy;
