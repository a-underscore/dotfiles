use crate::game::{Assets, Game};
use hex::{
    anyhow::{self, Context as _},
    context::Context3,
    renderers::ModelRenderer,
    vulkano::{
        Validated, VulkanError,
        command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage},
        swapchain::{SwapchainCreateInfo, SwapchainPresentInfo, acquire_next_image},
        sync::{self, GpuFuture},
    },
    winit::event::Event,
    world::World,
};
use std::sync::{Arc, RwLock};

/// Records and submits one frame.
pub fn frame(
    renderer: &Arc<RwLock<ModelRenderer>>,
    context: &Arc<RwLock<Context3>>,
    world: &Arc<RwLock<World>>,
    assets: &Assets,
    event: Event<()>,
    recreate_swapchain: &mut bool,
) -> anyhow::Result<()> {
    // `ModelRenderer::draw` can recreate the swapchain itself, but only after the
    // caller has already acquired an image from the old one, which invalidates the
    // acquire future. Doing it up front and handing `draw` a settled swapchain
    // avoids that.
    if *recreate_swapchain {
        if !recreate_render_targets(renderer, context, world, assets)? {
            return Ok(());
        }

        *recreate_swapchain = false;
    }

    let (image_index, suboptimal, acquire_future, mut builder) = {
        let mut context = context.write().unwrap();

        context
            .previous_frame_end
            .as_mut()
            .context("previous frame future missing")?
            .cleanup_finished();

        let (image_index, suboptimal, acquire_future) =
            match acquire_next_image(context.swapchain.clone(), None).map_err(Validated::unwrap) {
                Ok(frame) => frame,
                Err(VulkanError::OutOfDate) => {
                    *recreate_swapchain = true;

                    return Ok(());
                }
                Err(err) => return Err(err.into()),
            };
        let builder = AutoCommandBufferBuilder::primary(
            &context.command_buffer_allocator,
            context.queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )?;

        (image_index, suboptimal, acquire_future, builder)
    };

    ModelRenderer::draw(
        renderer.clone(),
        context.clone(),
        world.clone(),
        (
            hex::Control::new(event),
            image_index,
            suboptimal,
            &acquire_future,
            recreate_swapchain,
            &mut builder,
        ),
    )?;

    let command_buffer = builder.build()?;
    let mut context = context.write().unwrap();
    let future = context
        .previous_frame_end
        .take()
        .context("previous frame future missing")?
        .join(acquire_future)
        .then_execute(context.queue.clone(), command_buffer)?
        .then_swapchain_present(
            context.queue.clone(),
            SwapchainPresentInfo::swapchain_image_index(context.swapchain.clone(), image_index),
        )
        .then_signal_fence_and_flush();

    match future.map_err(Validated::unwrap) {
        Ok(future) => context.previous_frame_end = Some(future.boxed_send_sync()),
        Err(VulkanError::OutOfDate) => {
            *recreate_swapchain = true;
            context.previous_frame_end = Some(sync::now(context.device.clone()).boxed_send_sync());
        }
        Err(err) => {
            eprintln!("failed to present frame: {err}");
            context.previous_frame_end = Some(sync::now(context.device.clone()).boxed_send_sync());
        }
    }

    if suboptimal {
        *recreate_swapchain = true;
    }

    Ok(())
}

/// Rebuilds the swapchain, framebuffers and model pipelines. Returns `false`
/// when the window has no area, in which case there is nothing to draw.
fn recreate_render_targets(
    renderer: &Arc<RwLock<ModelRenderer>>,
    context: &Arc<RwLock<Context3>>,
    world: &Arc<RwLock<World>>,
    assets: &Assets,
) -> anyhow::Result<bool> {
    let models = Game::models(world);
    let mut context = context.write().unwrap();
    let image_extent: [u32; 2] = context.window.inner_size().into();

    if image_extent.contains(&0) {
        return Ok(false);
    }

    let (swapchain, images) = context.swapchain.recreate(SwapchainCreateInfo {
        image_extent,
        present_mode: context.present_mode,
        ..context.swapchain.create_info()
    })?;

    context.swapchain = swapchain;
    context.images = images;

    let mut renderer = renderer.write().unwrap();
    let (framebuffers, buffers, viewport) = ModelRenderer::window_size_dependant_setup(
        context.memory_allocator.clone(),
        &context.images,
        renderer.render_pass.clone(),
    )?;

    renderer.framebuffers = framebuffers;
    renderer.buffers = buffers;
    renderer.viewport = viewport;

    assets.refresh_pipelines(&context, &renderer, &models)?;

    Ok(true)
}
