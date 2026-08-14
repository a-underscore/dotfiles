use crate::util::{scaled_axis, yaw_pitch};
use hex::{
    components::Trans3,
    nalgebra::{UnitQuaternion, Vector2, Vector3},
    winit::keyboard::KeyCode,
};
use std::collections::HashSet;
use std::f32::consts::FRAC_PI_2;

const MOVE_SPEED: f32 = 6.0;
const SPRINT_SPEED: f32 = 10.0;
const LOOK_SPEED: f32 = 0.0032;
const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;

/// First-person controller stored as yaw/pitch plus a vertical velocity, so
/// looking around stays independent of movement. `update` only turns the view
/// and computes the horizontal wish velocity; the physics step in `Game`
/// integrates that wish (walls, gravity, jumping) into `position`.
pub struct FpsCamera {
    pub position: Vector3<f32>,
    pub yaw: f32,
    pub pitch: f32,
    pub vy: f32,
    pub grounded: bool,
    pub moving: bool,
    pub wish_velocity: Vector3<f32>,
    pub jump: bool,
}

impl FpsCamera {
    pub fn new(position: Vector3<f32>) -> Self {
        Self {
            position,
            yaw: 0.0,
            pitch: 0.0,
            vy: 0.0,
            grounded: true,
            moving: false,
            wish_velocity: Vector3::zeros(),
            jump: false,
        }
    }

    pub fn orientation(&self) -> UnitQuaternion<f32> {
        yaw_pitch(self.yaw, self.pitch)
    }

    pub fn forward(&self) -> Vector3<f32> {
        self.orientation() * -Vector3::z()
    }

    pub fn right(&self) -> Vector3<f32> {
        self.orientation() * Vector3::x()
    }

    pub fn update(&mut self, pressed: &HashSet<KeyCode>, mouse: Vector2<f32>) {
        self.yaw -= mouse.x * LOOK_SPEED;
        self.pitch = (self.pitch - mouse.y * LOOK_SPEED).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        let forward = self.forward();
        let right = self.right();

        let mut wish = Vector3::zeros();
        if pressed.contains(&KeyCode::KeyW) {
            wish += forward;
        }
        if pressed.contains(&KeyCode::KeyS) {
            wish -= forward;
        }
        if pressed.contains(&KeyCode::KeyA) {
            wish -= right;
        }
        if pressed.contains(&KeyCode::KeyD) {
            wish += right;
        }

        self.moving = wish != Vector3::zeros();
        self.wish_velocity = if self.moving {
            let speed = if pressed.contains(&KeyCode::ShiftLeft)
                || pressed.contains(&KeyCode::ShiftRight)
            {
                SPRINT_SPEED
            } else {
                MOVE_SPEED
            };

            wish.normalize() * speed
        } else {
            Vector3::zeros()
        };
        self.jump = pressed.contains(&KeyCode::Space);
    }

    pub fn apply(&self, trans: &mut Trans3) {
        trans.position = self.position;
        trans.rotation = scaled_axis(self.orientation());
    }
}
