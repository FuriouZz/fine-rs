pub use render;
pub use transform;
pub use math;
pub mod asset_loader;
pub mod camera;
pub mod frame;
pub mod init;
pub mod scene;
pub use camera::*;
pub use frame::*;

use init::*;

pub fn start<S: 'static + scene::Scene>() {
    #[cfg(target_arch = "wasm32")]
    {
        wasm_bindgen_futures::spawn_local(async move {
            let (event_loop, window, gpu, surface) = init_gpu_context().await;
            init_event_loop::<S>(event_loop, window, gpu, surface);
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let (event_loop, window, gpu, surface) = futures::executor::block_on(init_gpu_context());
        init_event_loop::<S>(event_loop, window, gpu, surface);
    }
}
