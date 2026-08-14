//! HEX FPS — a neon hex-arena first-person shooter.
//!
//! The weapon is *hitscan*: every shot is an instant raycast from the camera.
//! There are no projectiles — the tracer you see is purely cosmetic.

use std::f32::consts::{FRAC_PI_2, PI};

use bevy::{
    asset::RenderAssetUsages,
    core_pipeline::tonemapping::Tonemapping,
    input::mouse::AccumulatedMouseMotion,
    light::NotShadowCaster,
    math::bounding::{Aabb3d, RayCast3d},
    mesh::{Indices, PrimitiveTopology},
    post_process::bloom::Bloom,
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};

const HEX_SIZE: f32 = 1.0; // hexagon center-to-corner distance
const SQRT3: f32 = 1.7320508075688772;
const HEX_APOTHEM: f32 = HEX_SIZE * SQRT3 / 2.0;
const GRID_RADIUS: i32 = 5; // arena = all hexes with axial distance <= 5
const EYE_HEIGHT: f32 = 1.6;
const TARGET_SIZE: f32 = 0.72;
const TARGET_HEIGHT: f32 = 1.5;
const FIRE_RANGE: f32 = 90.0;
const FIRE_INTERVAL: f32 = 0.1; // 10 rounds per second
const PLAYER_SPEED: f32 = 6.0;
const SPRINT_SPEED: f32 = 10.0;
const JUMP_SPEED: f32 = 6.2;
const GRAVITY: f32 = 20.0;
const ARENA_RADIUS: f32 = 4.25; // max axial hex-distance for the player
const GUN_POS: Vec3 = Vec3::new(0.26, -0.24, -0.5);

fn main() {
    // Resolve the asset dir at compile time so textures are found whether the
    // game is launched via `cargo run` or the raw binary from target/.
    let asset_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .to_string_lossy()
        .into_owned();

    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: asset_dir,
            ..default()
        }).set(WindowPlugin {
            primary_window: Some(Window {
                title: "HEX FPS — hitscan arena".into(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(PlayerState::default())
        .insert_resource(FireControl::default())
        .insert_resource(Score::default())
        .insert_resource(HitMarker::default())
        .insert_resource(Recoil::default())
        .insert_resource(TargetSpawner::default())
        .insert_resource(WorldRng::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                cursor_control,
                mouse_look,
                player_move,
                gun_anim,
                shoot,
                target_spin,
                respawn_targets,
                tick_lifetimes,
                update_ui,
            ),
        )
        .run();
}

// ---------------------------------------------------------------------------
// Resources
// ---------------------------------------------------------------------------

#[derive(Resource, Default)]
struct PlayerState {
    yaw: f32,
    pitch: f32,
    vy: f32,
    grounded: bool,
    moving: bool,
}

#[derive(Resource)]
struct FireControl {
    cooldown: f32,
}
impl Default for FireControl {
    fn default() -> Self {
        Self { cooldown: 0.0 }
    }
}

#[derive(Resource)]
struct Score {
    value: u32,
    shots: u32,
    hits: u32,
}
impl Default for Score {
    fn default() -> Self {
        Self {
            value: 0,
            shots: 0,
            hits: 0,
        }
    }
}

#[derive(Resource)]
struct HitMarker {
    flash: f32,
}
impl Default for HitMarker {
    fn default() -> Self {
        Self { flash: 0.0 }
    }
}

#[derive(Resource)]
struct Recoil {
    amount: f32,
}
impl Default for Recoil {
    fn default() -> Self {
        Self { amount: 0.0 }
    }
}

#[derive(Resource)]
struct TargetSpawner {
    timer: f32,
    pending: u32,
}
impl Default for TargetSpawner {
    fn default() -> Self {
        Self {
            timer: 0.0,
            pending: 0,
        }
    }
}

#[derive(Resource)]
struct WorldRng {
    state: u64,
}
impl Default for WorldRng {
    fn default() -> Self {
        Self {
            state: 0x9E37_79B9_7F4A_7C15,
        }
    }
}
impl WorldRng {
    fn next_u32(&mut self) -> u32 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.state >> 33) as u32
    }
    fn range(&mut self, lo: u32, hi: u32) -> u32 {
        lo + self.next_u32() % (hi - lo)
    }
}

#[derive(Resource, Clone)]
struct GameAssets {
    target_mesh: Handle<Mesh>,
    target_material: Handle<StandardMaterial>,
    tracer_mesh: Handle<Mesh>,
    tracer_material: Handle<StandardMaterial>,
    impact_mesh: Handle<Mesh>,
    impact_material: Handle<StandardMaterial>,
    muzzle_mesh: Handle<Mesh>,
    muzzle_material: Handle<StandardMaterial>,
}

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

#[derive(Component)]
struct Player;

#[derive(Component)]
struct GunRoot;

#[derive(Component)]
struct Target {
    spin: f32,
}

#[derive(Component)]
struct Lifetime(f32);

#[derive(Component)]
struct CrosshairBar;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct StatsText;

// ---------------------------------------------------------------------------
// Setup
// ---------------------------------------------------------------------------

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // --- hex world ---------------------------------------------------------
    let floor_mesh = meshes.add(hex_prism(HEX_SIZE, 0.35));
    let wall_mesh = meshes.add(hex_prism(HEX_SIZE, 3.2));

    // CC0 textures (Poly Haven): applied via mesh UVs, emissive kept low so
    // the surface detail stays visible under the arena's neon lighting.
    let floor_tex = asset_server.load("textures/metal_plate_02_diff_1k.jpg");
    let wall_tex = asset_server.load("textures/metal_plate_diff_1k.jpg");

    let floor_a = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.9, 0.95),
        base_color_texture: Some(floor_tex.clone()),
        emissive: LinearRgba::rgb(0.05, 0.11, 0.13),
        ..default()
    });
    let floor_b = materials.add(StandardMaterial {
        base_color: Color::srgb(0.6, 0.7, 0.8),
        base_color_texture: Some(floor_tex.clone()),
        emissive: LinearRgba::rgb(0.02, 0.05, 0.08),
        ..default()
    });
    let wall_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.7, 0.95),
        base_color_texture: Some(wall_tex),
        emissive: LinearRgba::rgb(0.10, 0.03, 0.16),
        ..default()
    });

    for q in -GRID_RADIUS..=GRID_RADIUS {
        let r_min = (-GRID_RADIUS).max(-q - GRID_RADIUS);
        let r_max = GRID_RADIUS.min(-q + GRID_RADIUS);
        for r in r_min..=r_max {
            let d = hex_dist(q, r);
            let is_wall = d == GRID_RADIUS;
            let (x, z) = axial_to_world(q as f32, r as f32);
            let y = if is_wall { 3.2 / 2.0 } else { -0.35 / 2.0 };
            let mesh = if is_wall {
                wall_mesh.clone()
            } else {
                floor_mesh.clone()
            };
            let mat = if is_wall {
                wall_mat.clone()
            } else if (q + r).rem_euclid(2) == 0 {
                floor_a.clone()
            } else {
                floor_b.clone()
            };
            commands.spawn((
                Mesh3d(mesh),
                MeshMaterial3d(mat),
                Transform::from_xyz(x, y, z),
            ));
        }
    }

    // --- transient assets --------------------------------------------------
    let target_mesh = meshes.add(hex_prism(TARGET_SIZE, TARGET_HEIGHT));
    let target_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.85, 0.10, 0.16),
        emissive: LinearRgba::rgb(0.6, 0.06, 0.10),
        ..default()
    });
    let tracer_mesh = meshes.add(hex_prism(0.016, 1.0));
    let tracer_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.05, 0.12, 0.12),
        emissive: LinearRgba::rgb(2.0, 7.0, 7.0),
        ..default()
    });
    let impact_mesh = meshes.add(Sphere::new(0.07).mesh().ico(3).unwrap());
    let impact_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.10, 0.05, 0.01),
        emissive: LinearRgba::rgb(8.0, 4.0, 0.6),
        ..default()
    });
    let muzzle_mesh = meshes.add(Sphere::new(0.10).mesh().ico(3).unwrap());
    let muzzle_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.12, 0.10, 0.02),
        emissive: LinearRgba::rgb(9.0, 7.0, 1.5),
        ..default()
    });

    let assets = GameAssets {
        target_mesh: target_mesh.clone(),
        target_material: target_mat.clone(),
        tracer_mesh: tracer_mesh.clone(),
        tracer_material: tracer_mat.clone(),
        impact_mesh: impact_mesh.clone(),
        impact_material: impact_mat.clone(),
        muzzle_mesh: muzzle_mesh.clone(),
        muzzle_material: muzzle_mat.clone(),
    };
    commands.insert_resource(assets.clone());

    // --- targets -----------------------------------------------------------
    let mut rng = WorldRng::default();
    for (q, r) in [(0, -2), (2, -1), (-2, 1)] {
        let (x, z) = axial_to_world(q as f32, r as f32);
        spawn_target(
            &mut commands,
            &assets,
            Vec3::new(x, TARGET_HEIGHT / 2.0, z),
            &mut rng,
        );
    }

    // --- player + camera + viewmodel gun -----------------------------------
    let gun_body_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.14, 0.15, 0.19),
        ..default()
    });
    let gun_glow_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.05, 0.10, 0.10),
        emissive: LinearRgba::rgb(1.5, 4.0, 4.0),
        ..default()
    });
    let gun_accent_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.10, 0.05, 0.10),
        emissive: LinearRgba::rgb(4.0, 0.8, 4.0),
        ..default()
    });

    let body = meshes.add(Cuboid::new(0.09, 0.12, 0.42));
    let grip = meshes.add(Cuboid::new(0.055, 0.14, 0.08));
    let barrel = meshes.add(hex_prism(0.035, 0.32));
    let sight = meshes.add(Cuboid::new(0.02, 0.05, 0.07));
    let under = meshes.add(Cuboid::new(0.035, 0.03, 0.18));

    commands
        .spawn((Player, Transform::from_xyz(0.0, EYE_HEIGHT, 3.6), Visibility::default()))
        .with_children(|p| {
            p.spawn((
                Camera3d::default(),
                Camera {
                    clear_color: ClearColorConfig::Custom(Color::srgb(0.008, 0.012, 0.03)),
                    ..default()
                },
                Projection::from(PerspectiveProjection {
                    fov: 85.0_f32.to_radians(),
                    ..default()
                }),
                Msaa::Sample4,
                Bloom {
                    intensity: 0.32,
                    ..default()
                },
                Tonemapping::TonyMcMapface,
                DistanceFog {
                    color: Color::srgb(0.008, 0.012, 0.03),
                    falloff: FogFalloff::Linear {
                        start: 28.0,
                        end: 75.0,
                    },
                    ..default()
                },
            ))
            .with_children(|c| {
                c.spawn((
                    GunRoot,
                    Transform::from_translation(GUN_POS),
                    Visibility::default(),
                ))
                .with_children(|g| {
                        g.spawn((
                            Mesh3d(body.clone()),
                            MeshMaterial3d(gun_body_mat.clone()),
                            Transform::from_xyz(0.0, 0.0, -0.06),
                        ));
                        g.spawn((
                            Mesh3d(grip.clone()),
                            MeshMaterial3d(gun_body_mat.clone()),
                            Transform::from_xyz(0.0, -0.12, 0.10),
                        ));
                        g.spawn((
                            Mesh3d(barrel.clone()),
                            MeshMaterial3d(gun_glow_mat.clone()),
                            Transform::from_xyz(0.0, 0.01, -0.30)
                                .with_rotation(Quat::from_rotation_x(FRAC_PI_2)),
                        ));
                        g.spawn((
                            Mesh3d(sight.clone()),
                            MeshMaterial3d(gun_accent_mat.clone()),
                            Transform::from_xyz(0.0, 0.085, -0.16),
                        ));
                        g.spawn((
                            Mesh3d(under.clone()),
                            MeshMaterial3d(gun_accent_mat.clone()),
                            Transform::from_xyz(0.0, -0.02, -0.18),
                        ));
                    });
            });
        });

    // --- lights ------------------------------------------------------------
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.9, 0.87, 1.0),
            illuminance: 25_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::new(0.3, -1.0, 0.8), Vec3::Y),
    ));
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.35, 0.4, 0.6),
        brightness: 140.0,
        ..default()
    });

    // --- HUD ---------------------------------------------------------------
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: px(16.0),
            left: px(20.0),
            ..default()
        },
        children![(
            ScoreText,
            Text::new("SCORE 000000"),
            TextFont {
                font_size: FontSize::Px(30.0),
                ..default()
            },
            TextColor(Color::srgb(0.4, 1.0, 0.9)),
        )],
    ));
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: px(16.0),
            right: px(20.0),
            ..default()
        },
        children![(
            StatsText,
            Text::new(""),
            TextFont {
                font_size: FontSize::Px(18.0),
                ..default()
            },
            TextColor(Color::srgb(0.55, 0.75, 1.0)),
        )],
    ));
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: px(18.0),
            right: px(20.0),
            ..default()
        },
        children![(
            Text::new("HEX-9 // HITSCAN BLASTER\n∞ ROUNDS · INSTANT HIT"),
            TextFont {
                font_size: FontSize::Px(20.0),
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.4, 0.9)),
        )],
    ));
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: px(18.0),
            left: px(20.0),
            ..default()
        },
        children![(
            Text::new("WASD MOVE · LSHIFT SPRINT · SPACE JUMP\nLMB FIRE · ESC FREE MOUSE"),
            TextFont {
                font_size: FontSize::Px(15.0),
                ..default()
            },
            TextColor(Color::srgb(0.7, 0.75, 0.9)),
        )],
    ));

    // crosshair (container pinned to screen center)
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(50.0),
            width: px(0.0),
            height: px(0.0),
            ..default()
        },
        children![
            (
                CrosshairBar,
                Node {
                    position_type: PositionType::Absolute,
                    left: px(-17.0),
                    top: px(-1.5),
                    width: px(34.0),
                    height: px(3.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.35, 1.0, 0.9)),
            ),
            (
                CrosshairBar,
                Node {
                    position_type: PositionType::Absolute,
                    left: px(-1.5),
                    top: px(-17.0),
                    width: px(3.0),
                    height: px(34.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.35, 1.0, 0.9)),
            ),
            (
                Node {
                    position_type: PositionType::Absolute,
                    left: px(-2.0),
                    top: px(-2.0),
                    width: px(4.0),
                    height: px(4.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(1.0, 1.0, 1.0)),
            ),
        ],
    ));
}

fn spawn_target(commands: &mut Commands, assets: &GameAssets, pos: Vec3, rng: &mut WorldRng) {
    commands.spawn((
        Target {
            spin: 0.5 + rng.range(0, 50) as f32 / 100.0,
        },
        Mesh3d(assets.target_mesh.clone()),
        MeshMaterial3d(assets.target_material.clone()),
        Transform::from_translation(pos),
    ));
}

// ---------------------------------------------------------------------------
// Systems
// ---------------------------------------------------------------------------

fn cursor_control(
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut cursor: Single<&mut CursorOptions>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        cursor.visible = false;
        cursor.grab_mode = CursorGrabMode::Locked;
    }
    if keys.just_pressed(KeyCode::Escape) {
        cursor.visible = true;
        cursor.grab_mode = CursorGrabMode::None;
    }
}

fn mouse_look(
    mouse: Res<AccumulatedMouseMotion>,
    cursor: Single<&CursorOptions>,
    mut state: ResMut<PlayerState>,
    mut player: Single<&mut Transform, With<Player>>,
    mut camera: Single<&mut Transform, (With<Camera3d>, Without<Player>)>,
) {
    if cursor.grab_mode != CursorGrabMode::Locked {
        return;
    }
    let delta = mouse.delta;
    if delta != Vec2::ZERO {
        state.yaw -= delta.x * 0.0032;
        state.pitch = (state.pitch - delta.y * 0.0032).clamp(-FRAC_PI_2 + 0.01, FRAC_PI_2 - 0.01);
    }
    player.rotation = Quat::from_rotation_y(state.yaw);
    camera.rotation = Quat::from_rotation_x(state.pitch);
}

fn player_move(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<PlayerState>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    let dt = time.delta_secs();
    let (sy, cy) = state.yaw.sin_cos();
    let forward = Vec3::new(-sy, 0.0, -cy);
    let right = Vec3::new(cy, 0.0, -sy);

    let mut wish = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        wish += forward;
    }
    if keys.pressed(KeyCode::KeyS) {
        wish -= forward;
    }
    if keys.pressed(KeyCode::KeyA) {
        wish -= right;
    }
    if keys.pressed(KeyCode::KeyD) {
        wish += right;
    }
    state.moving = wish != Vec3::ZERO;
    if state.moving {
        let speed = if keys.pressed(KeyCode::ShiftLeft) {
            SPRINT_SPEED
        } else {
            PLAYER_SPEED
        };
        let step = wish.normalize() * speed * dt;
        player.translation.x += step.x;
        player.translation.z += step.z;
    }

    if keys.just_pressed(KeyCode::Space) && state.grounded {
        state.vy = JUMP_SPEED;
        state.grounded = false;
    }
    if !state.grounded {
        state.vy -= GRAVITY * dt;
        player.translation.y += state.vy * dt;
        if player.translation.y <= EYE_HEIGHT {
            player.translation.y = EYE_HEIGHT;
            state.vy = 0.0;
            state.grounded = true;
        }
    }

    // clamp the player inside the hexagonal arena
    let (q, r) = world_to_axial(player.translation.x, player.translation.z);
    let d = hex_dist_frac(q, r);
    if d > ARENA_RADIUS {
        let k = ARENA_RADIUS / d;
        let (x, z) = axial_to_world(q * k, r * k);
        player.translation.x = x;
        player.translation.z = z;
    }
}

fn gun_anim(
    time: Res<Time>,
    state: Res<PlayerState>,
    mut recoil: ResMut<Recoil>,
    mut gun: Single<&mut Transform, With<GunRoot>>,
) {
    let dt = time.delta_secs();
    recoil.amount = (recoil.amount - dt * 7.0).max(0.0);
    let t = time.elapsed_secs();
    let bobbing = state.moving && state.grounded;
    let bob = if bobbing { (t * 10.5).sin() } else { 0.0 };
    let sway = if bobbing {
        (t * 7.0).sin() * 0.5 + 0.5
    } else {
        0.0
    };
    gun.translation = Vec3::new(
        GUN_POS.x + bob * 0.010,
        GUN_POS.y - (bob * 0.008).abs() + sway * 0.004,
        GUN_POS.z + recoil.amount * 0.085,
    );
    gun.rotation = Quat::from_rotation_x(-recoil.amount * 0.12) * Quat::from_rotation_z(bob * 0.02);
}

fn shoot(
    time: Res<Time>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    camera: Single<&GlobalTransform, With<Camera3d>>,
    targets: Query<(Entity, &GlobalTransform, &Target)>,
    mut fire: ResMut<FireControl>,
    mut score: ResMut<Score>,
    mut hitmarker: ResMut<HitMarker>,
    mut recoil: ResMut<Recoil>,
    mut spawner: ResMut<TargetSpawner>,
    assets: Res<GameAssets>,
) {
    fire.cooldown -= time.delta_secs();
    if !mouse.pressed(MouseButton::Left) || fire.cooldown > 0.0 {
        return;
    }
    fire.cooldown = FIRE_INTERVAL;
    score.shots += 1;
    recoil.amount = 1.0;

    // --- hitscan ray -------------------------------------------------------
    let origin = camera.translation();
    let dir = Dir3::new(camera.rotation() * Vec3::NEG_Z).unwrap_or(Dir3::NEG_Z);
    let ray = RayCast3d::new(origin, dir, FIRE_RANGE);

    let mut best: Option<(f32, Entity)> = None;
    let half = Vec3A::new(
        TARGET_SIZE * 0.95,
        TARGET_HEIGHT / 2.0,
        TARGET_SIZE * 0.95 * HEX_APOTHEM / HEX_SIZE,
    );
    for (entity, gt, _) in &targets {
        let aabb = Aabb3d::new(Vec3A::from(gt.translation()), half);
        if let Some(dist) = ray.aabb_intersection_at(&aabb) {
            if best.is_none_or(|(bd, _)| dist < bd) {
                best = Some((dist, entity));
            }
        }
    }

    let muzzle = origin + *dir * 0.65;
    let hit_point = match best {
        Some((dist, entity)) => {
            let p = origin + *dir * dist;
            score.value += 100;
            score.hits += 1;
            hitmarker.flash = 0.18;
            spawner.pending += 1;
            spawner.timer = spawner.timer.max(0.7);
            commands.entity(entity).despawn();
            p
        }
        None => {
            // no target: the ray still lands on the floor if aiming down
            if dir.y < -0.02 {
                origin + *dir * (-origin.y / dir.y)
            } else {
                origin + *dir * FIRE_RANGE
            }
        }
    };

    // --- tracer beam -------------------------------------------------------
    let len = (hit_point - muzzle).length();
    if len > 0.05 {
        commands.spawn((
            Mesh3d(assets.tracer_mesh.clone()),
            MeshMaterial3d(assets.tracer_material.clone()),
            Transform {
                translation: (muzzle + hit_point) / 2.0,
                rotation: Quat::from_rotation_arc(Vec3::Y, (hit_point - muzzle).normalize()),
                scale: Vec3::new(1.0, len, 1.0),
            },
            NotShadowCaster,
            Lifetime(0.09),
        ));
    }

    // --- muzzle flash ------------------------------------------------------
    commands.spawn((
        Mesh3d(assets.muzzle_mesh.clone()),
        MeshMaterial3d(assets.muzzle_material.clone()),
        Transform::from_translation(muzzle),
        NotShadowCaster,
        Lifetime(0.06),
    ));
    commands.spawn((
        PointLight {
            color: Color::srgb(1.0, 0.75, 0.35),
            intensity: 400_000.0,
            range: 7.0,
            ..default()
        },
        Transform::from_translation(muzzle),
        Lifetime(0.05),
    ));

    // --- impact ------------------------------------------------------------
    commands.spawn((
        Mesh3d(assets.impact_mesh.clone()),
        MeshMaterial3d(assets.impact_material.clone()),
        Transform::from_translation(hit_point),
        NotShadowCaster,
        Lifetime(0.14),
    ));
    if best.is_none() {
        commands.spawn((
            PointLight {
                color: Color::srgb(0.5, 0.9, 1.0),
                intensity: 150_000.0,
                range: 4.0,
                ..default()
            },
            Transform::from_translation(hit_point),
            Lifetime(0.10),
        ));
    }
}

fn target_spin(time: Res<Time>, mut targets: Query<(&mut Transform, &Target)>) {
    let dt = time.delta_secs();
    for (mut t, target) in &mut targets {
        t.rotate_y(target.spin * dt);
    }
}

fn respawn_targets(
    time: Res<Time>,
    mut commands: Commands,
    mut spawner: ResMut<TargetSpawner>,
    mut rng: ResMut<WorldRng>,
    assets: Res<GameAssets>,
) {
    if spawner.pending == 0 {
        return;
    }
    spawner.timer -= time.delta_secs();
    if spawner.timer > 0.0 {
        return;
    }
    spawner.pending -= 1;
    spawner.timer = 0.7;
    loop {
        let q = rng.range(0, 7) as i32 - 3;
        let r = rng.range(0, 7) as i32 - 3;
        if (1..=3).contains(&hex_dist(q, r)) {
            let (x, z) = axial_to_world(q as f32, r as f32);
            commands.spawn((
                Target {
                    spin: 0.5 + rng.range(0, 50) as f32 / 100.0,
                },
                Mesh3d(assets.target_mesh.clone()),
                MeshMaterial3d(assets.target_material.clone()),
                Transform::from_xyz(x, TARGET_HEIGHT / 2.0, z),
            ));
            break;
        }
    }
}

fn tick_lifetimes(time: Res<Time>, mut commands: Commands, mut q: Query<(Entity, &mut Lifetime)>) {
    let dt = time.delta_secs();
    for (entity, mut life) in &mut q {
        life.0 -= dt;
        if life.0 <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn update_ui(
    time: Res<Time>,
    score: Res<Score>,
    mut hitmarker: ResMut<HitMarker>,
    mut texts: Query<(&mut Text, Option<&ScoreText>, Option<&StatsText>)>,
    mut bars: Query<&mut BackgroundColor, With<CrosshairBar>>,
) {
    hitmarker.flash = (hitmarker.flash - time.delta_secs()).max(0.0);
    let acc = if score.shots > 0 {
        100.0 * score.hits as f32 / score.shots as f32
    } else {
        100.0
    };
    for (mut text, is_score, is_stats) in &mut texts {
        if is_score.is_some() {
            text.0 = format!("SCORE {:06}", score.value);
        } else if is_stats.is_some() {
            text.0 = format!(
                "SHOTS {}\nHITS  {}\nACC   {:.0}%",
                score.shots, score.hits, acc
            );
        }
    }
    let color = if hitmarker.flash > 0.0 {
        Color::srgb(1.0, 0.12, 0.2)
    } else {
        Color::srgb(0.35, 1.0, 0.9)
    };
    for mut bar in &mut bars {
        bar.0 = color;
    }
}

// ---------------------------------------------------------------------------
// Hex math + mesh generation
// ---------------------------------------------------------------------------

/// Axial hex coordinates -> world (x, z). Uses flat-top orientation.
fn axial_to_world(q: f32, r: f32) -> (f32, f32) {
    let x = HEX_SIZE * 1.5 * q;
    let z = HEX_SIZE * 3.0_f32.sqrt() * (r + q * 0.5);
    (x, z)
}

/// World (x, z) -> fractional axial hex coordinates.
fn world_to_axial(x: f32, z: f32) -> (f32, f32) {
    let q = x / (HEX_SIZE * 1.5);
    let r = z / (HEX_SIZE * 3.0_f32.sqrt()) - q * 0.5;
    (q, r)
}

/// Hex distance for fractional coordinates (max(|q|, |r|, |q+r|)).
fn hex_dist_frac(q: f32, r: f32) -> f32 {
    q.abs().max(r.abs()).max((q + r).abs())
}

fn hex_dist(q: i32, r: i32) -> i32 {
    q.abs().max(r.abs()).max((q + r).abs())
}

/// A hexagonal prism: flat-top hexagon extruded along Y, centered at origin.
/// `size` is the center-to-corner distance, `height` the extrusion length.
fn hex_prism(size: f32, height: f32) -> Mesh {
    let h = height / 2.0;
    let corners: Vec<[f32; 2]> = (0..6)
        .map(|i| {
            let a = i as f32 * PI / 3.0;
            [size * a.cos(), size * a.sin()]
        })
        .collect();

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    fn push(
        positions: &mut Vec<[f32; 3]>,
        normals: &mut Vec<[f32; 3]>,
        uvs: &mut Vec<[f32; 2]>,
        p: [f32; 3],
        n: [f32; 3],
        uv: [f32; 2],
    ) -> u32 {
        positions.push(p);
        normals.push(n);
        uvs.push(uv);
        (positions.len() - 1) as u32
    }

    let top_center = push(
        &mut positions,
        &mut normals,
        &mut uvs,
        [0.0, h, 0.0],
        [0.0, 1.0, 0.0],
        [0.5, 0.5],
    );
    let top: Vec<u32> = corners
        .iter()
        .map(|c| {
            push(
                &mut positions,
                &mut normals,
                &mut uvs,
                [c[0], h, c[1]],
                [0.0, 1.0, 0.0],
                [0.5 + c[0] / (2.0 * size), 0.5 + c[1] / (2.0 * size)],
            )
        })
        .collect();
    for i in 0..6 {
        let n = (i + 1) % 6;
        indices.extend_from_slice(&[top_center, top[i], top[n]]);
    }

    let bottom_center = push(
        &mut positions,
        &mut normals,
        &mut uvs,
        [0.0, -h, 0.0],
        [0.0, -1.0, 0.0],
        [0.5, 0.5],
    );
    let bottom: Vec<u32> = corners
        .iter()
        .map(|c| {
            push(
                &mut positions,
                &mut normals,
                &mut uvs,
                [c[0], -h, c[1]],
                [0.0, -1.0, 0.0],
                [0.5 + c[0] / (2.0 * size), 0.5 + c[1] / (2.0 * size)],
            )
        })
        .collect();
    for i in 0..6 {
        let n = (i + 1) % 6;
        indices.extend_from_slice(&[bottom_center, bottom[n], bottom[i]]);
    }

    for i in 0..6 {
        let n = (i + 1) % 6;
        let c0 = corners[i];
        let c1 = corners[n];
        let nx = c0[0] + c1[0];
        let nz = c0[1] + c1[1];
        let l = (nx * nx + nz * nz).sqrt();
        let normal = [nx / l, 0.0, nz / l];

        let a = push(
            &mut positions,
            &mut normals,
            &mut uvs,
            [c0[0], h, c0[1]],
            normal,
            [0.0, 1.0],
        );
        let b = push(
            &mut positions,
            &mut normals,
            &mut uvs,
            [c0[0], -h, c0[1]],
            normal,
            [0.0, 0.0],
        );
        let c = push(
            &mut positions,
            &mut normals,
            &mut uvs,
            [c1[0], -h, c1[1]],
            normal,
            [1.0, 0.0],
        );
        let d = push(
            &mut positions,
            &mut normals,
            &mut uvs,
            [c1[0], h, c1[1]],
            normal,
            [1.0, 1.0],
        );
        indices.extend_from_slice(&[a, b, c]);
        indices.extend_from_slice(&[a, c, d]);
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices))
}
