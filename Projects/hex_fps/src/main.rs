mod camera;
mod game;
mod mesh;
mod physics;
mod render;
mod util;

use camera::FpsCamera;
use game::{CAMERA_TAG, Game};
use hex::{
    anyhow::{self, Context as _},
    components::{Camera3, Tag, Trans3},
    context::Context3,
    nalgebra::{Vector2, Vector3, Vector4},
    renderers::ModelRenderer,
    vulkano::swapchain::PresentMode,
    winit::{
        dpi::LogicalSize,
        event::{DeviceEvent, ElementState, Event, MouseButton, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        keyboard::{KeyCode, PhysicalKey},
        window::{CursorGrabMode, WindowBuilder},
    },
    world::{EntityManager, World},
};
use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

const CONTROLS: &str = "\
HEX FPS — hitscan arena on the hex engine
  W/A/S/D      move, Shift sprint, Space jump
  mouse        look around (click to lock the cursor)
  LMB          fire, hold for automatic
  escape       quit";

fn main() -> anyhow::Result<()> {
    println!("{CONTROLS}");

    let event_loop = EventLoop::new()?;

    event_loop.set_control_flow(ControlFlow::Poll);

    let window = Arc::new(
        WindowBuilder::new()
            .with_title("HEX FPS — hitscan arena")
            .with_inner_size(LogicalSize::new(1280.0, 720.0))
            .build(&event_loop)?,
    );
    let context = Context3::new(&event_loop, window.clone(), PresentMode::Fifo)?;
    let renderer = {
        let context = context.read().unwrap();

        ModelRenderer::new_with_antialiasing(&context, Vector4::new(0.008, 0.012, 0.03, 1.0), 4)?
    };
    let world = World::new(EntityManager::new(), Vector3::new(0.30, 0.35, 0.55), 0.12);
    let mut camera = FpsCamera::new(Vector3::new(0.0, 1.6, 3.6));
    let mut game = {
        let context = context.read().unwrap();
        let renderer = renderer.read().unwrap();

        Game::new(&world, &context, &renderer)?
    };

    let mut pressed: HashSet<KeyCode> = HashSet::new();
    let mut mouse_delta = Vector2::zeros();
    let mut fire_held = false;
    let mut last_frame = Instant::now();
    let mut recreate_swapchain = false;
    let mut cursor_locked = false;

    event_loop.run(move |event, elwt| {
        match &event {
            Event::WindowEvent { window_id, event } if *window_id == window.id() => match event {
                WindowEvent::CloseRequested => {
                    elwt.exit();

                    return;
                }
                WindowEvent::Resized(_) => {
                    recreate_swapchain = true;

                    if let Err(err) = update_camera_aspect(&context, &world) {
                        eprintln!("failed to update camera aspect: {err:#}");
                        elwt.exit();

                        return;
                    }
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    let PhysicalKey::Code(code) = event.physical_key else {
                        return;
                    };

                    match event.state {
                        ElementState::Pressed => {
                            pressed.insert(code);

                            if event.repeat {
                                return;
                            }

                            if code == KeyCode::Escape {
                                elwt.exit();

                                return;
                            }
                        }
                        ElementState::Released => {
                            pressed.remove(&code);
                        }
                    }
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    if *button == MouseButton::Left {
                        fire_held = *state == ElementState::Pressed;

                        if fire_held && !cursor_locked {
                            let _ = window.set_cursor_grab(CursorGrabMode::Locked);

                            window.set_cursor_visible(false);
                            cursor_locked = true;
                        }
                    }
                }
                _ => {}
            },
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                mouse_delta.x += delta.0 as f32;
                mouse_delta.y += delta.1 as f32;
            }
            Event::AboutToWait => {
                window.request_redraw();

                return;
            }
            _ => return,
        }

        let Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } = &event
        else {
            return;
        };

        // Clamped so that a hitch cannot hand the simulation an enormous delta.
        let delta_time = last_frame
            .elapsed()
            .min(Duration::from_millis(100))
            .as_secs_f32();

        last_frame = Instant::now();

        let look = mouse_delta;

        mouse_delta = Vector2::zeros();

        camera.update(&pressed, look);

        if let Err(err) = game.update(&world, &mut camera, fire_held, delta_time) {
            eprintln!("game update failed: {err:#}");
            elwt.exit();

            return;
        }

        if let Err(err) = apply_camera(&world, &camera) {
            eprintln!("failed to update camera: {err:#}");
            elwt.exit();

            return;
        }

        if let Err(err) = render::frame(
            &renderer,
            &context,
            &world,
            &game.assets,
            event.clone(),
            &mut recreate_swapchain,
        ) {
            eprintln!("failed to draw frame: {err:#}");
            elwt.exit();
        }
    })?;

    Ok(())
}

fn apply_camera(world: &Arc<RwLock<World>>, rig: &FpsCamera) -> anyhow::Result<()> {
    let em = world.read().unwrap().em.clone();
    let em = em.read().unwrap();
    let camera = Tag(CAMERA_TAG.into())
        .find(&em)
        .context("camera entity not found")?;
    let trans = em
        .get_component::<Trans3>(camera)
        .context("camera transform not found")?;

    rig.apply(&mut trans.write().unwrap());

    Ok(())
}

fn update_camera_aspect(
    context: &Arc<RwLock<Context3>>,
    world: &Arc<RwLock<World>>,
) -> anyhow::Result<()> {
    let size = context.read().unwrap().window.inner_size();
    let aspect = size.width as f32 / size.height.max(1) as f32;
    let em = world.read().unwrap().em.clone();
    let em = em.read().unwrap();
    let camera = Tag(CAMERA_TAG.into())
        .find(&em)
        .context("camera entity not found")?;
    let camera = em
        .get_component::<Camera3>(camera)
        .context("camera component not found")?;

    camera.write().unwrap().set_aspect(aspect);

    Ok(())
}
