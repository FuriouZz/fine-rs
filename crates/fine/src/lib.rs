pub mod render {
    pub use fine_render::*;
}
pub mod transform {
    pub use fine_transform::*;
}

pub mod math {
    pub use fine_math::*;
}

pub mod asset_loader;
pub mod camera;
pub mod frame;
pub mod init;
pub mod mesh;
pub mod scene;
pub use camera::*;
pub use frame::*;
pub use mesh::*;

use init::*;

pub fn start<S: 'static + scene::Scene>() {
    // #[cfg(target_arch = "wasm32")]
    // {
    //     wasm_bindgen_futures::spawn_local(async move || {
    //         let (event_loop, window, gpu, surface) = init_gpu_context().await;
    //         init_event_loop::<S>(event_loop, window, gpu, surface);
    //     });
    // }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let (event_loop, window, gpu, surface) = futures::executor::block_on(init_gpu_context());
        init_event_loop::<S>(event_loop, window, gpu, surface);
    }
}
