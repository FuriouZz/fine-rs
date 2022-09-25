use std::time::{Duration, Instant};
use std::future::Future;

use crate::frame::Frame;
use crate::render::context::{Context, Surface};
use crate::scene::Scene;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

#[cfg(not(target_arch = "wasm32"))]
pub struct Spawner<'a> {
    executor: async_executor::LocalExecutor<'a>,
}

#[cfg(not(target_arch = "wasm32"))]
impl<'a> Spawner<'a> {
    fn new() -> Self {
        Self {
            executor: async_executor::LocalExecutor::new(),
        }
    }

    #[allow(dead_code)]
    pub fn spawn_local(&self, future: impl Future<Output = ()> + 'a) {
        self.executor.spawn(future).detach();
    }

    fn run_until_stalled(&self) {
        while self.executor.try_tick() {}
    }
}

#[cfg(target_arch = "wasm32")]
pub struct Spawner {}

#[cfg(target_arch = "wasm32")]
impl Spawner {
    fn new() -> Self {
        Self {}
    }

    #[allow(dead_code)]
    pub fn spawn_local(&self, future: impl Future<Output = ()> + 'static) {
        wasm_bindgen_futures::spawn_local(future);
    }
}

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

    let spawner = Spawner::new();

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
                spawner.run_until_stalled();

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
