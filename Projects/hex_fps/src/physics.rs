use crate::{camera::FpsCamera, util::scaled_axis};
use hex::{
    Id,
    components::Trans3,
    nalgebra::{Isometry3, Point3, Translation3, Vector3},
    world::World,
};
use rapier3d::{
    control::KinematicCharacterController,
    dynamics::{
        CCDSolver, ImpulseJointSet, IntegrationParameters, IslandManager, MultibodyJointSet,
        RigidBodyBuilder, RigidBodyHandle, RigidBodySet, RigidBodyType,
    },
    geometry::{
        BroadPhaseMultiSap, ColliderBuilder, ColliderHandle, ColliderSet, NarrowPhase, Ray,
        SharedShape,
    },
    pipeline::{PhysicsPipeline, QueryFilter, QueryPipeline},
};
use std::{
    collections::HashSet,
    f32::consts::PI,
    sync::{Arc, RwLock},
};

/// World constants shared with the movement controller.
pub const EYE_HEIGHT: f32 = 1.6;
pub const GRAVITY: f32 = 20.0;
pub const JUMP_SPEED: f32 = 6.2;

const PLAYER_RADIUS: f32 = 0.3;
const PLAYER_HALF_HEIGHT: f32 = 0.55;

/// The rapier handles behind an entity that lives in the physics world.
#[derive(Clone)]
pub struct Body {
    pub rigid: RigidBodyHandle,
    pub collider: ColliderHandle,
}

/// Owns the rapier world: the arena colliders, the spinning targets, and the
/// kinematic character controller that moves the player through it.
pub struct Physics {
    gravity: Vector3<f32>,
    pipeline: PhysicsPipeline,
    query: QueryPipeline,
    integration: IntegrationParameters,
    islands: IslandManager,
    broad_phase: BroadPhaseMultiSap,
    narrow_phase: NarrowPhase,
    bodies: RigidBodySet,
    colliders: ColliderSet,
    impulse_joints: ImpulseJointSet,
    multibody_joints: MultibodyJointSet,
    ccd_solver: CCDSolver,
    controller: KinematicCharacterController,
    player_shape: SharedShape,
    /// Offset from the character's feet to the capsule centre.
    player_shape_center: f32,
    /// Colliders that shots can hit (everything else, i.e. the arena, is ignored).
    shootable: HashSet<ColliderHandle>,
}

impl Physics {
    pub fn new() -> Self {
        Self {
            gravity: Vector3::new(0.0, -GRAVITY, 0.0),
            pipeline: PhysicsPipeline::new(),
            query: QueryPipeline::new(),
            integration: IntegrationParameters::default(),
            islands: IslandManager::new(),
            broad_phase: BroadPhaseMultiSap::new(),
            narrow_phase: NarrowPhase::new(),
            bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            impulse_joints: ImpulseJointSet::new(),
            multibody_joints: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            controller: KinematicCharacterController::default(),
            player_shape: SharedShape::capsule_y(PLAYER_HALF_HEIGHT, PLAYER_RADIUS),
            player_shape_center: PLAYER_HALF_HEIGHT + PLAYER_RADIUS,
            shootable: HashSet::new(),
        }
    }

    /// Advances the simulation: kinematic targets spin, struck targets tumble.
    pub fn step(&mut self, dt: f32) {
        self.integration.dt = dt;

        self.pipeline.step(
            &self.gravity,
            &self.integration,
            &mut self.islands,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.impulse_joints,
            &mut self.multibody_joints,
            &mut self.ccd_solver,
            Some(&mut self.query),
            &(),
            &(),
        );
    }

    /// Convex hull matching the renderer's hex prism: same corners, extruded
    /// along Y, so colliders line up exactly with the visuals.
    pub fn hex_prism_shape(size: f32, height: f32) -> SharedShape {
        let h = height / 2.0;
        let mut points = Vec::with_capacity(12);

        for i in 0..6 {
            let a = i as f32 * PI / 3.0;

            points.push(Point3::new(size * a.cos(), -h, size * a.sin()));
            points.push(Point3::new(size * a.cos(), h, size * a.sin()));
        }

        SharedShape::convex_hull(&points).expect("hex prism vertices are not coplanar")
    }

    /// A fixed collider for the arena floor or walls. Not tied to an entity.
    pub fn add_static_collider(&mut self, position: Vector3<f32>, shape: SharedShape) {
        let body = self.bodies.insert(
            RigidBodyBuilder::fixed()
                .position(Isometry3::translation(position.x, position.y, position.z))
                .build(),
        );

        self.colliders.insert_with_parent(
            ColliderBuilder::new(shape).build(),
            body,
            &mut self.bodies,
        );
    }

    /// A kinematic target body that spins in place and can be shot. The
    /// collider's `user_data` carries the entity id so ray hits can be mapped
    /// back to the game object.
    pub fn add_target_body(
        &mut self,
        shape: SharedShape,
        position: Vector3<f32>,
        spin: f32,
        entity: Id,
    ) -> Body {
        let rigid = self.bodies.insert(
            RigidBodyBuilder::kinematic_velocity_based()
                .position(Isometry3::translation(position.x, position.y, position.z))
                .angvel(Vector3::new(0.0, spin, 0.0))
                .build(),
        );
        let collider = self.colliders.insert_with_parent(
            ColliderBuilder::new(shape)
                .user_data(entity as u128)
                .build(),
            rigid,
            &mut self.bodies,
        );

        self.shootable.insert(collider);

        Body { rigid, collider }
    }

    /// Removes a body and its collider, e.g. when a struck target despawns.
    pub fn remove_body(&mut self, body: &Body) {
        self.shootable.remove(&body.collider);
        self.colliders
            .remove(body.collider, &mut self.islands, &mut self.bodies, true);
        self.bodies.remove(
            body.rigid,
            &mut self.islands,
            &mut self.colliders,
            &mut self.impulse_joints,
            &mut self.multibody_joints,
            true,
        );
    }

    /// Integrates the player: jump, gravity and collisions with the arena and
    /// the targets, all resolved by the kinematic character controller.
    pub fn move_player(&mut self, camera: &mut FpsCamera, dt: f32) {
        if camera.jump && camera.grounded {
            camera.vy = JUMP_SPEED;
            camera.grounded = false;
        }
        if !camera.grounded {
            camera.vy -= GRAVITY * dt;
        }

        let mut translation = camera.wish_velocity * dt;
        translation.y += camera.vy * dt;

        let feet = camera.position - Vector3::y() * EYE_HEIGHT;
        let mut pos = Isometry3::translation(feet.x, feet.y + self.player_shape_center, feet.z);
        let movement = self.controller.move_shape(
            dt,
            &self.bodies,
            &self.colliders,
            &self.query,
            &*self.player_shape,
            &pos,
            translation,
            QueryFilter::default(),
            |_| {},
        );

        camera.grounded = movement.grounded;
        if camera.grounded {
            camera.vy = 0.0;
        }

        pos *= Translation3::from(movement.translation);
        camera.position = Vector3::new(
            pos.translation.x,
            pos.translation.y - self.player_shape_center + EYE_HEIGHT,
            pos.translation.z,
        );
    }

    /// Hitscan against target colliders only. Returns (distance, entity id).
    pub fn raycast(
        &self,
        origin: Vector3<f32>,
        dir: Vector3<f32>,
        max_distance: f32,
    ) -> Option<(f32, Id)> {
        let predicate = |handle: ColliderHandle, _: &rapier3d::geometry::Collider| {
            self.shootable.contains(&handle)
        };
        let filter = QueryFilter::default().predicate(&predicate);
        let ray = Ray::new(Point3::from(origin), dir);

        self.query
            .cast_ray(
                &self.bodies,
                &self.colliders,
                &ray,
                max_distance,
                true,
                filter,
            )
            .map(|(handle, toi)| (toi, self.colliders[handle].user_data as Id))
    }

    /// Knocks a target out: it becomes dynamic, so the impulse sends it
    /// tumbling through the arena instead of desyncing from its collider.
    pub fn strike(&mut self, body: &Body, impulse: Vector3<f32>, spin: Vector3<f32>) {
        self.shootable.remove(&body.collider);

        let rigid = &mut self.bodies[body.rigid];

        rigid.set_body_type(RigidBodyType::Dynamic, true);
        rigid.recompute_mass_properties_from_colliders(&self.colliders);
        rigid.set_gravity_scale(1.0, true);
        rigid.set_angvel(spin, true);
        rigid.apply_impulse(impulse, true);
    }

    /// Copies every physics body's transform back onto its entity's `Trans3`.
    pub fn sync_bodies(&self, world: &Arc<RwLock<World>>) {
        let em = world.read().unwrap().em.clone();
        let em = em.write().unwrap();
        let entities: Vec<Id> = em.entities().collect();

        for entity in entities {
            let Some(body) = em.get_component::<Body>(entity) else {
                continue;
            };
            let body = body.read().unwrap();
            let Some(rigid) = self.bodies.get(body.rigid) else {
                continue;
            };
            let Some(trans) = em.get_component::<Trans3>(entity) else {
                continue;
            };
            let mut trans = trans.write().unwrap();

            trans.position = rigid.position().translation.vector;
            trans.rotation = scaled_axis(rigid.position().rotation);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::camera::FpsCamera;

    const DT: f32 = 1.0 / 60.0;

    fn step(physics: &mut Physics, camera: &mut FpsCamera, seconds: f32) {
        for _ in 0..(seconds / DT) as usize {
            physics.step(DT);
            physics.move_player(camera, DT);
        }
    }

    /// A strip of floor hexes along -Z plus a wall hex at (0, -5), mirroring
    /// the arena layout along that corridor.
    fn corridor(physics: &mut Physics) {
        for r in 0..=2 {
            let z = -1.732_050_8 * r as f32;

            physics.add_static_collider(
                Vector3::new(0.0, -0.175, z),
                Physics::hex_prism_shape(1.0, 0.35),
            );
        }
        physics.add_static_collider(
            Vector3::new(0.0, 1.6, -5.0),
            Physics::hex_prism_shape(1.0, 3.2),
        );
    }

    #[test]
    fn player_is_blocked_by_wall() {
        let mut physics = Physics::new();

        corridor(&mut physics);

        let mut camera = FpsCamera::new(Vector3::new(0.0, 1.6, 0.0));

        camera.wish_velocity = Vector3::new(0.0, 0.0, -6.0);
        step(&mut physics, &mut camera, 1.0);

        // The wall's near face is at z = -4.134 and the capsule has a 0.3
        // radius, so the eye should stop just short of it — not pass through.
        assert!(
            (-4.2..-3.5).contains(&camera.position.z),
            "player walked through the wall: z = {}",
            camera.position.z
        );
        assert!(
            camera.position.y > 1.5,
            "player fell through the floor: y = {}",
            camera.position.y
        );
    }

    #[test]
    fn raycast_hits_target_and_reports_entity() {
        let mut physics = Physics::new();

        physics.add_target_body(
            Physics::hex_prism_shape(0.72, 1.5),
            Vector3::new(0.0, 0.75, -5.0),
            0.0,
            42,
        );
        physics.step(DT);

        // Start below the target's 1.5 m top so a level ray actually meets it.
        let (distance, entity) = physics
            .raycast(
                Vector3::new(0.0, 1.2, 0.0),
                Vector3::new(0.0, 0.0, -1.0),
                20.0,
            )
            .expect("ray should hit the target");

        assert_eq!(entity, 42);
        assert!((4.0..5.0).contains(&distance), "distance = {distance}");
    }

    #[test]
    fn kinematic_target_spins_in_place() {
        let mut physics = Physics::new();

        let body = physics.add_target_body(
            Physics::hex_prism_shape(0.72, 1.5),
            Vector3::new(0.0, 0.75, 0.0),
            2.0,
            7,
        );
        physics.step(1.0);

        let rotation = physics.bodies[body.rigid].position().rotation;

        assert!(
            (rotation.angle() - 2.0).abs() < 0.05,
            "expected ~2 rad of spin, got {}",
            rotation.angle()
        );
    }

    #[test]
    fn struck_target_becomes_dynamic_and_unshootable() {
        let mut physics = Physics::new();

        physics.add_static_collider(
            Vector3::new(0.0, -0.175, 0.0),
            Physics::hex_prism_shape(1.0, 0.35),
        );
        let body = physics.add_target_body(
            Physics::hex_prism_shape(0.72, 1.5),
            Vector3::new(0.0, 0.75, 0.0),
            1.0,
            7,
        );
        physics.strike(
            &body,
            Vector3::new(0.0, 2.0, 0.0),
            Vector3::new(1.0, 1.0, 1.0),
        );

        assert_eq!(
            physics.bodies[body.rigid].body_type(),
            RigidBodyType::Dynamic
        );
        assert!(
            physics
                .raycast(
                    Vector3::new(0.0, 1.2, 0.0),
                    Vector3::new(0.0, 0.0, -1.0),
                    20.0
                )
                .is_none(),
            "struck target should no longer be shootable"
        );

        physics.step(0.5);

        // It popped up under its impulse, so it must have moved off the spawn.
        let y = physics.bodies[body.rigid].position().translation.y;

        assert!(
            (y - 0.75).abs() > 0.05,
            "struck target did not move: y = {y}"
        );
    }
}
