use hex::nalgebra::{UnitQuaternion, Vector3};

/// `Trans3::rotation` is fed to `Matrix4::new_rotation`, which interprets the
/// vector as a *scaled axis* (direction = axis, length = angle) rather than as
/// Euler angles. Building rotations as quaternions and converting here keeps
/// composed rotations correct instead of accidentally introducing roll.
pub fn scaled_axis(rotation: UnitQuaternion<f32>) -> Vector3<f32> {
    rotation.scaled_axis()
}

/// Orientation whose local `-Z` points from `eye` towards `target`.
pub fn look_at(eye: Vector3<f32>, target: Vector3<f32>) -> UnitQuaternion<f32> {
    let forward = target - eye;

    if forward.norm() < f32::EPSILON {
        return UnitQuaternion::identity();
    }

    let up = if forward.normalize().y.abs() > 0.999 {
        Vector3::z()
    } else {
        Vector3::y()
    };

    UnitQuaternion::face_towards(&(-forward), &up)
}

/// Yaw around world +Y applied after pitch around local +X.
pub fn yaw_pitch(yaw: f32, pitch: f32) -> UnitQuaternion<f32> {
    UnitQuaternion::from_axis_angle(&Vector3::y_axis(), yaw)
        * UnitQuaternion::from_axis_angle(&Vector3::x_axis(), pitch)
}

// ---------------------------------------------------------------------------
// Hex-grid math (flat-top orientation, same layout as the arena builder)
// ---------------------------------------------------------------------------

pub const HEX_SIZE: f32 = 1.0;
pub const SQRT3: f32 = 1.7320508075688772;

/// Axial hex coordinates -> world (x, z).
pub fn axial_to_world(q: f32, r: f32) -> (f32, f32) {
    let x = HEX_SIZE * 1.5 * q;
    let z = HEX_SIZE * SQRT3 * (r + q * 0.5);

    (x, z)
}

/// Deterministic xorshift, so the arena spawns the same way every run without
/// pulling in a random number generator dependency.
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self(seed | 1)
    }

    pub fn next_f32(&mut self) -> f32 {
        let mut x = self.0;

        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;

        (x >> 40) as f32 / (1u32 << 24) as f32
    }

    pub fn range(&mut self, low: f32, high: f32) -> f32 {
        low + self.next_f32() * (high - low)
    }

    pub fn range_i32(&mut self, low: i32, high: i32) -> i32 {
        low + (self.next_f32() * (high - low) as f32) as i32
    }
}

/// Rotates a vector so the mesh's local +Y axis points along `dir`, then
/// returns it as a scaled-axis `Trans3` rotation. Used to aim the tracer
/// prism from the muzzle towards the hit point.
pub fn align_y_to(dir: Vector3<f32>) -> Vector3<f32> {
    let dir = if dir.norm_squared() < f32::EPSILON {
        Vector3::y()
    } else {
        dir.normalize()
    };

    scaled_axis(UnitQuaternion::rotation_between(&Vector3::y(), &dir).unwrap_or_default())
}
