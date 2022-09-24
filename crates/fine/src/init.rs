use std::time::{Duration, Instant};

use crate::frame::Frame;
use crate::render::context::{Context, Surface};
use crate::scene::Scene;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub(crate) async fn init_gpu_context() -> (EventLoop<()>, Window, Context, Surface) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let (gpu, surface) = Context::new(&window, &Default::default()).await.unwrap();
    (event_loop, window, gpu, surface)
}

pub(crate) fn init_event_loop<S: 'static + Scene>(
    event_loop: EventLoop<()>,
    window: Window,
    gpu: Context,
    surface: Surface,
) {
    let mut gpu = gpu;
    let mut surface = surface;

    let mut scene = S::on_load(Frame {
        context: &mut gpu,
        surface: &mut surface,
    });

    let size = window.inner_size();
    surface.resize(&gpu, size.width, size.height);

    #[cfg(not(target_arch = "wasm32"))]
    let mut last_update_inst = Instant::now();

    // winit has window.current_monitor().video_modes() but that is a list of all full screen video modes.
    // So without extra dependencies it's a bit tricky to get the max refresh rate we can run the window on.
    // Therefore we just go with 60fps - sorry 120hz+ folks!
    #[cfg(not(target_arch = "wasm32"))]
    let target_frametime = Duration::from_secs_f64(1.0 / 60.0);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            // On close
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,

            // On resize
            Event::WindowEvent {
                event: WindowEvent::Resized(ref size),
                window_id,
            } if window_id == window.id() => {
                surface.resize(&gpu, size.width, size.height);
            }

            Event::RedrawEventsCleared => {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    // Clamp to some max framerate to avoid busy-looping too much
                    // (we might be in wgpu::PresentMode::Mailbox, thus discarding superfluous frames)
                    let time_since_last_frame = last_update_inst.elapsed();
                    if time_since_last_frame >= target_frametime {
                        window.request_redraw();
                        last_update_inst = Instant::now();
                    } else {
                        *control_flow = ControlFlow::WaitUntil(
                            Instant::now() + target_frametime - time_since_last_frame,
                        );
                    }
                }

                #[cfg(target_arch = "wasm32")]
                window.request_redraw();
            }

            // On request redraw
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                scene.on_draw(Frame {
                    context: &mut gpu,
                    surface: &mut surface,
                });
                gpu.submit_queue(&mut surface);
            }

            // Discard other events
            _ => (),
        }
    });
}
