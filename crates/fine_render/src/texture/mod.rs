mod atlas;
mod texture;
mod raw;
pub use atlas::TextureAtlas;
pub use texture::Texture;
pub use raw::RawTexture;
use crate::context::Context;

pub trait AsTextureView {
    fn as_view(&self) -> &wgpu::TextureView;
}

pub fn create_texture_color(
    color: u32,
    width: u32,
    height: u32,
    gpu: &mut Context,
) -> RawTexture {
    let bytes: Vec<u8> = (0..width * height)
        .flat_map(|_index| {
            std::iter::once((color & 0xFF) as u8)
                .chain(std::iter::once((color >> 8 & 0xFF) as u8))
                .chain(std::iter::once((color >> 16 & 0xFF) as u8))
                .chain(std::iter::once(0xFF))
        })
        .collect();

    RawTexture::from_bytes(gpu, width, height, wgpu::TextureUsages::COPY_SRC, &bytes)
}
