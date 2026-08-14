use crate::{
    camera::FpsCamera,
    mesh,
    physics::{Body, Physics},
    util::{HEX_SIZE, Rng, align_y_to, axial_to_world, look_at, scaled_axis},
};
use hex::{
    Id, anyhow,
    components::{Camera3, Light3, Model, Tag, Trans3},
    context::Context3,
    nalgebra::{UnitQuaternion, Vector3, Vector4},
    renderers::ModelRenderer,
    world::World,
};
use std::{
    f32::consts::FRAC_PI_2,
    sync::{Arc, RwLock},
};

pub const CAMERA_TAG: &str = "camera";

const GRID_RADIUS: i32 = 5;
const TARGET_SIZE: f32 = 0.72;
const TARGET_HEIGHT: f32 = 1.5;
const FIRE_RANGE: f32 = 90.0;
const FIRE_INTERVAL: f32 = 0.1;
const RESPAWN_DELAY: f32 = 0.7;
/// How long a shot target tumbles through the arena before despawning.
const STRIKE_LINGER: f32 = 0.55;
const FOV: f32 = 70.0_f32.to_radians();

const FLOOR_A: Vector4<f32> = Vector4::new(0.10, 0.22, 0.26, 1.0);
const FLOOR_B: Vector4<f32> = Vector4::new(0.06, 0.12, 0.18, 1.0);
const WALL_COLOR: Vector4<f32> = Vector4::new(0.30, 0.10, 0.34, 1.0);
const TARGET_COLOR: Vector4<f32> = Vector4::new(0.85, 0.10, 0.16, 1.0);
const GUN_BODY: Vector4<f32> = Vector4::new(0.20, 0.22, 0.28, 1.0);
const GUN_GLOW: Vector4<f32> = Vector4::new(0.10, 0.50, 0.50, 1.0);
const GUN_ACCENT: Vector4<f32> = Vector4::new(0.58, 0.16, 0.64, 1.0);
const TRACER_COLOR: Vector4<f32> = Vector4::new(0.15, 0.85, 0.85, 1.0);
const IMPACT_COLOR: Vector4<f32> = Vector4::new(0.95, 0.55, 0.15, 1.0);
const MUZZLE_COLOR: Vector4<f32> = Vector4::new(0.98, 0.88, 0.45, 1.0);

fn hex_dist(q: i32, r: i32) -> i32 {
    q.abs().max(r.abs()).max((q + r).abs())
}

/// A viewmodel part: a model entity anchored to the camera with a fixed local
/// offset and orientation.
struct GunPart {
    entity: Id,
    local: Vector3<f32>,
    rotation: UnitQuaternion<f32>,
}

/// The meshes and pipelines shared by every model in the game. `Model` bakes
/// the viewport into its pipelines, so they must be rebuilt whenever the
/// swapchain is; rebuilding the prototypes and handing the results to every
/// instance keeps the `Arc`s shared, which is what the renderer batches on.
pub struct Assets {
    floor: Arc<RwLock<Model>>,
    wall: Arc<RwLock<Model>>,
    target: Arc<RwLock<Model>>,
    gun_body: Arc<RwLock<Model>>,
    gun_glow: Arc<RwLock<Model>>,
    gun_accent: Arc<RwLock<Model>>,
    tracer: Arc<RwLock<Model>>,
    impact: Arc<RwLock<Model>>,
    muzzle: Arc<RwLock<Model>>,
}

impl Assets {
    pub fn new(context: &Context3, renderer: &ModelRenderer) -> anyhow::Result<Self> {
        let white = Vector4::new(1.0, 1.0, 1.0, 1.0);

        Ok(Self {
            floor: Model::solid(
                context,
                renderer,
                mesh::hex_prism(HEX_SIZE, 0.35, context)?,
                white,
            )?,
            wall: Model::solid(
                context,
                renderer,
                mesh::hex_prism(HEX_SIZE, 3.2, context)?,
                white,
            )?,
            target: Model::solid(
                context,
                renderer,
                mesh::hex_prism(TARGET_SIZE, TARGET_HEIGHT, context)?,
                white,
            )?,
            gun_body: Model::solid(context, renderer, mesh::cube(context)?, white)?,
            gun_glow: Model::solid(
                context,
                renderer,
                mesh::hex_prism(0.035, 0.32, context)?,
                white,
            )?,
            gun_accent: Model::solid(context, renderer, mesh::cube(context)?, white)?,
            tracer: Model::solid(
                context,
                renderer,
                mesh::hex_prism(0.016, 1.0, context)?,
                white,
            )?,
            impact: Model::solid(context, renderer, mesh::sphere(2, 6, context)?, white)?,
            muzzle: Model::solid(context, renderer, mesh::sphere(2, 6, context)?, white)?,
        })
    }

    fn prototypes(&self) -> [&Arc<RwLock<Model>>; 9] {
        [
            &self.floor,
            &self.wall,
            &self.target,
            &self.gun_body,
            &self.gun_glow,
            &self.gun_accent,
            &self.tracer,
            &self.impact,
            &self.muzzle,
        ]
    }

    pub fn refresh_pipelines(
        &self,
        context: &Context3,
        renderer: &ModelRenderer,
        instances: &[Arc<RwLock<Model>>],
    ) -> anyhow::Result<()> {
        for prototype in self.prototypes() {
            prototype
                .write()
                .unwrap()
                .recreate_pipeline(context, renderer)?;
        }

        for instance in instances {
            let mesh = Arc::as_ptr(&instance.read().unwrap().mesh);
            let Some(prototype) = self
                .prototypes()
                .into_iter()
                .find(|p| Arc::as_ptr(&p.read().unwrap().mesh) == mesh)
            else {
                continue;
            };
            let pipelines = {
                let prototype = prototype.read().unwrap();

                (
                    prototype.pipeline.clone(),
                    prototype.lighting_pipeline.clone(),
                    prototype.point_lighting_pipeline.clone(),
                    prototype.shadow_pipeline.clone(),
                )
            };
            let mut instance = instance.write().unwrap();

            instance.pipeline = pipelines.0;
            instance.lighting_pipeline = pipelines.1;
            instance.point_lighting_pipeline = pipelines.2;
            instance.shadow_pipeline = pipelines.3;
        }

        Ok(())
    }
}

/// Clones a prototype and recolours the copy. The clone shares the prototype's
/// mesh and pipeline handles, so the renderer still groups them into a single
/// instanced draw while `Instance3` supplies the per-model colour.
fn tint(prototype: &Arc<RwLock<Model>>, color: Vector4<f32>) -> Arc<RwLock<Model>> {
    let mut model = prototype.read().unwrap().clone();

    model.color = color;

    Arc::new(RwLock::new(model))
}

pub struct Game {
    pub assets: Assets,
    physics: Physics,
    score: u32,
    shots: u32,
    hits: u32,
    fire_cooldown: f32,
    respawn_timer: f32,
    pending_respawns: u32,
    /// Short-lived effects (tracers, impacts, muzzle flashes).
    transient: Vec<(Id, f32)>,
    /// Targets knocked out by shots, tumbling until they despawn.
    struck: Vec<(Id, f32)>,
    gun: Vec<GunPart>,
    recoil: f32,
    time: f32,
    rng: Rng,
}

impl Game {
    pub fn new(
        world: &Arc<RwLock<World>>,
        context: &Context3,
        renderer: &ModelRenderer,
    ) -> anyhow::Result<Self> {
        let size = context.window.inner_size();
        let aspect = size.width as f32 / size.height.max(1) as f32;
        let mut game = Self {
            assets: Assets::new(context, renderer)?,
            physics: Physics::new(),
            score: 0,
            shots: 0,
            hits: 0,
            fire_cooldown: 0.0,
            respawn_timer: 0.0,
            pending_respawns: 0,
            transient: Vec::new(),
            struck: Vec::new(),
            gun: Vec::new(),
            recoil: 0.0,
            time: 0.0,
            rng: Rng::new(0x9E37_79B9_7F4A_7C15),
        };

        {
            let em = world.read().unwrap().em.clone();
            let mut em = em.write().unwrap();

            let camera = em.add(true);

            em.add_component(camera, Tag::new(CAMERA_TAG));
            em.add_component(camera, Camera3::new(aspect, FOV, 0.1, 250.0));
            em.add_component(
                camera,
                Trans3::new(
                    Vector3::new(0.0, 1.6, 3.6),
                    Vector3::zeros(),
                    Vector3::repeat(1.0),
                ),
            );

            for (position, color, intensity) in [
                (
                    Vector3::new(0.0, 18.0, 14.0),
                    Vector3::new(1.0, 0.93, 0.82),
                    0.45,
                ),
                (
                    Vector3::new(0.0, 16.0, -14.0),
                    Vector3::new(0.55, 0.68, 1.0),
                    0.26,
                ),
            ] {
                let light = em.add(true);

                em.add_component(
                    light,
                    Light3::spot(
                        context,
                        renderer,
                        color * intensity,
                        0.35,
                        42.0,
                        55.0_f32.to_radians().cos(),
                        0.5,
                        90.0,
                    )?,
                );
                em.add_component(
                    light,
                    Trans3::new(
                        position,
                        scaled_axis(look_at(position, Vector3::new(0.0, 0.0, 0.0))),
                        Vector3::repeat(1.0),
                    ),
                );
            }
        }

        game.build_arena(world);
        game.spawn_gun(world);

        for (q, r) in [(0, -2), (2, -1), (-2, 1)] {
            let (x, z) = axial_to_world(q as f32, r as f32);

            game.spawn_target(world, Vector3::new(x, TARGET_HEIGHT / 2.0, z));
        }

        Ok(game)
    }

    /// Static floor and ring of walls. These get a `Trans3` once and never
    /// move; their rapier colliders keep the player inside the arena.
    fn build_arena(&mut self, world: &Arc<RwLock<World>>) {
        let em = world.read().unwrap().em.clone();
        let mut em = em.write().unwrap();

        for q in -GRID_RADIUS..=GRID_RADIUS {
            let r_min = (-GRID_RADIUS).max(-q - GRID_RADIUS);
            let r_max = GRID_RADIUS.min(-q + GRID_RADIUS);

            for r in r_min..=r_max {
                let is_wall = hex_dist(q, r) == GRID_RADIUS;
                let (x, z) = axial_to_world(q as f32, r as f32);
                let y = if is_wall { 3.2 / 2.0 } else { -0.35 / 2.0 };
                let color = if is_wall {
                    WALL_COLOR
                } else if (q + r).rem_euclid(2) == 0 {
                    FLOOR_A
                } else {
                    FLOOR_B
                };
                let model = if is_wall {
                    tint(&self.assets.wall, color)
                } else {
                    tint(&self.assets.floor, color)
                };
                let entity = em.add(true);

                em.add_component(entity, model);
                em.add_component(
                    entity,
                    Trans3::new(
                        Vector3::new(x, y, z),
                        Vector3::zeros(),
                        Vector3::repeat(1.0),
                    ),
                );

                let height = if is_wall { 3.2 } else { 0.35 };

                self.physics.add_static_collider(
                    Vector3::new(x, y, z),
                    Physics::hex_prism_shape(HEX_SIZE, height),
                );
            }
        }
    }

    fn spawn_gun(&mut self, world: &Arc<RwLock<World>>) {
        let em = world.read().unwrap().em.clone();
        let mut em = em.write().unwrap();

        let mut add = |em: &mut hex::world::entity_manager::EntityManager,
                       model: Arc<RwLock<Model>>,
                       local: Vector3<f32>,
                       rotation: UnitQuaternion<f32>,
                       scale: Vector3<f32>| {
            let entity = em.add(true);

            em.add_component(entity, model);
            em.add_component(
                entity,
                Trans3::new(Vector3::zeros(), Vector3::zeros(), scale),
            );

            self.gun.push(GunPart {
                entity,
                local,
                rotation,
            });
        };

        add(
            &mut em,
            tint(&self.assets.gun_body, GUN_BODY),
            Vector3::new(0.0, 0.0, -0.06),
            UnitQuaternion::identity(),
            Vector3::new(0.09, 0.12, 0.42),
        );
        add(
            &mut em,
            tint(&self.assets.gun_body, GUN_BODY),
            Vector3::new(0.0, -0.12, 0.10),
            UnitQuaternion::identity(),
            Vector3::new(0.055, 0.14, 0.08),
        );
        add(
            &mut em,
            tint(&self.assets.gun_glow, GUN_GLOW),
            Vector3::new(0.0, 0.01, -0.30),
            UnitQuaternion::from_axis_angle(&Vector3::x_axis(), -FRAC_PI_2),
            Vector3::repeat(1.0),
        );
        add(
            &mut em,
            tint(&self.assets.gun_accent, GUN_ACCENT),
            Vector3::new(0.0, 0.085, -0.16),
            UnitQuaternion::identity(),
            Vector3::new(0.02, 0.05, 0.07),
        );
        add(
            &mut em,
            tint(&self.assets.gun_accent, GUN_ACCENT),
            Vector3::new(0.0, -0.02, -0.18),
            UnitQuaternion::identity(),
            Vector3::new(0.035, 0.03, 0.18),
        );
    }

    fn spawn_target(&mut self, world: &Arc<RwLock<World>>, position: Vector3<f32>) {
        let em = world.read().unwrap().em.clone();
        let mut em = em.write().unwrap();
        let entity = em.add(true);
        let body = self.physics.add_target_body(
            Physics::hex_prism_shape(TARGET_SIZE, TARGET_HEIGHT),
            position,
            0.5 + self.rng.range(0.0, 0.5),
            entity,
        );

        em.add_component(entity, Arc::new(RwLock::new(body)));
        em.add_component(entity, tint(&self.assets.target, TARGET_COLOR));
        em.add_component(
            entity,
            Trans3::new(position, Vector3::zeros(), Vector3::repeat(1.0)),
        );
    }

    fn spawn_transient(
        &mut self,
        world: &Arc<RwLock<World>>,
        model: Arc<RwLock<Model>>,
        position: Vector3<f32>,
        rotation: Vector3<f32>,
        scale: Vector3<f32>,
        lifetime: f32,
    ) {
        let em = world.read().unwrap().em.clone();
        let mut em = em.write().unwrap();
        let entity = em.add(true);

        em.add_component(entity, model);
        em.add_component(entity, Trans3::new(position, rotation, scale));

        self.transient.push((entity, lifetime));
    }

    pub fn update(
        &mut self,
        world: &Arc<RwLock<World>>,
        camera: &mut FpsCamera,
        fire_held: bool,
        dt: f32,
    ) -> anyhow::Result<()> {
        self.time += dt;
        self.fire_cooldown -= dt;

        if fire_held && self.fire_cooldown <= 0.0 {
            self.shoot(world, camera);
        }

        self.physics.step(dt);
        self.physics.move_player(camera, dt);
        self.physics.sync_bodies(world);
        self.respawn_targets(world, dt);
        self.tick_transient(world, dt);
        self.tick_struck(world, dt);
        self.attach_gun(world, camera, dt);

        Ok(())
    }

    fn shoot(&mut self, world: &Arc<RwLock<World>>, camera: &FpsCamera) {
        self.fire_cooldown = FIRE_INTERVAL;
        self.shots += 1;
        self.recoil = 1.0;

        let origin = camera.position;
        let dir = camera.forward().normalize();
        let muzzle = origin + dir * 0.65;
        let hit_point = match self.physics.raycast(origin, dir, FIRE_RANGE) {
            Some((distance, entity)) => {
                self.score += 100;
                self.hits += 1;
                self.pending_respawns += 1;
                self.respawn_timer = self.respawn_timer.max(RESPAWN_DELAY);

                let hit = origin + dir * distance;

                // The target becomes dynamic and gets knocked flying, then
                // despawns once `tick_struck` runs out its timer.
                let impulse = dir * 2.5 + Vector3::y() * 2.0;
                let spin = Vector3::new(
                    self.rng.range(-6.0, 6.0),
                    self.rng.range(-6.0, 6.0),
                    self.rng.range(-6.0, 6.0),
                );
                {
                    let em = world.read().unwrap().em.clone();
                    let em = em.read().unwrap();

                    if let Some(body) = em.get_component::<Body>(entity) {
                        self.physics.strike(&body.read().unwrap(), impulse, spin);
                    }
                }
                self.struck.push((entity, STRIKE_LINGER));

                println!(
                    "SCORE {:06}  SHOTS {}  HITS {}  ACC {:.0}%",
                    self.score,
                    self.shots,
                    self.hits,
                    100.0 * self.hits as f32 / self.shots as f32
                );

                hit
            }
            None => {
                if dir.y < -0.02 {
                    origin + dir * (-origin.y / dir.y)
                } else {
                    origin + dir * FIRE_RANGE
                }
            }
        };

        // Tracer beam from muzzle to hit point.
        let delta = hit_point - muzzle;
        let length = delta.norm();

        if length > 0.05 {
            self.spawn_transient(
                world,
                tint(&self.assets.tracer, TRACER_COLOR),
                (muzzle + hit_point) / 2.0,
                align_y_to(delta),
                Vector3::new(1.0, length, 1.0),
                0.09,
            );
        }

        self.spawn_transient(
            world,
            tint(&self.assets.muzzle, MUZZLE_COLOR),
            muzzle,
            Vector3::zeros(),
            Vector3::repeat(0.20),
            0.06,
        );
        self.spawn_transient(
            world,
            tint(&self.assets.impact, IMPACT_COLOR),
            hit_point,
            Vector3::zeros(),
            Vector3::repeat(0.14),
            0.14,
        );
    }

    fn respawn_targets(&mut self, world: &Arc<RwLock<World>>, dt: f32) {
        if self.pending_respawns == 0 {
            return;
        }

        self.respawn_timer -= dt;
        if self.respawn_timer > 0.0 {
            return;
        }

        self.pending_respawns -= 1;
        self.respawn_timer = RESPAWN_DELAY;

        let (q, r) = loop {
            let q = self.rng.range_i32(-3, 4);
            let r = self.rng.range_i32(-3, 4);

            if (1..=3).contains(&hex_dist(q, r)) {
                break (q, r);
            }
        };
        let (x, z) = axial_to_world(q as f32, r as f32);

        self.spawn_target(world, Vector3::new(x, TARGET_HEIGHT / 2.0, z));
    }

    fn tick_transient(&mut self, world: &Arc<RwLock<World>>, dt: f32) {
        let em = world.read().unwrap().em.clone();
        let mut em = em.write().unwrap();
        let mut i = 0;

        while i < self.transient.len() {
            let (entity, life) = self.transient[i];

            if life - dt <= 0.0 {
                em.rm(entity);
                self.transient.swap_remove(i);
            } else {
                self.transient[i].1 = life - dt;
                i += 1;
            }
        }
    }

    /// Despawns knocked-out targets (and their bodies) once their tumbling
    /// time is up.
    fn tick_struck(&mut self, world: &Arc<RwLock<World>>, dt: f32) {
        let em = world.read().unwrap().em.clone();
        let mut em = em.write().unwrap();
        let mut i = 0;

        while i < self.struck.len() {
            let (entity, life) = self.struck[i];

            if life - dt <= 0.0 {
                if let Some(body) = em.get_component::<Body>(entity) {
                    self.physics.remove_body(&body.read().unwrap());
                }

                em.rm(entity);
                self.struck.swap_remove(i);
            } else {
                self.struck[i].1 = life - dt;
                i += 1;
            }
        }
    }

    /// Anchors the viewmodel to the camera: local offsets rotated into world
    /// space, with bobbing while moving and a recoil kick after each shot.
    fn attach_gun(&mut self, world: &Arc<RwLock<World>>, camera: &FpsCamera, dt: f32) {
        self.recoil = (self.recoil - dt * 7.0).max(0.0);

        let orient = camera.orientation();
        let eye = camera.position;
        let bob = if camera.moving && camera.grounded {
            (self.time * 10.5).sin()
        } else {
            0.0
        };
        let sway = if camera.moving && camera.grounded {
            (self.time * 7.0).sin() * 0.5 + 0.5
        } else {
            0.0
        };
        let bob_offset = Vector3::new(
            bob * 0.010,
            -(bob * 0.008).abs() + sway * 0.004,
            self.recoil * 0.085,
        );
        let kick = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), self.recoil * 0.12);

        let em = world.read().unwrap().em.clone();
        let em = em.write().unwrap();

        for part in &self.gun {
            let position = eye + orient * (part.local + bob_offset);
            let rotation = orient * part.rotation * kick;

            if let Some(trans) = em.get_component::<Trans3>(part.entity) {
                let mut trans = trans.write().unwrap();

                trans.position = position;
                trans.rotation = scaled_axis(rotation);
            }
        }
    }

    /// Every live `Model` component, used to refresh pipelines after a resize.
    pub fn models(world: &Arc<RwLock<World>>) -> Vec<Arc<RwLock<Model>>> {
        let em = world.read().unwrap().em.clone();
        let em = em.read().unwrap();

        em.entities()
            .filter_map(|entity| em.get_component::<Model>(entity))
            .collect()
    }
}
