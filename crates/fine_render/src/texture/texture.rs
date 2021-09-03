use super::RawTexture;
use std::num::NonZeroU32;
use crate::context::Context;

pub struct Texture {
    view: wgpu::TextureView,
    width: u32,
    height: u32,
}

impl Texture {
    pub fn from_raw(raw: &RawTexture, width: u32, height: u32) -> Self {
        // Create texture view
        let view = raw.as_raw().create_view(&wgpu::TextureViewDescriptor {
            label: None,
            format: Some(wgpu::TextureFormat::Bgra8Unorm),
            dimension: Some(wgpu::TextureViewDimension::D2),
            aspect: wgpu::TextureAspect::default(),
            base_mip_level: 0,
            mip_level_count: NonZeroU32::new(1),
            base_array_layer: 0,
            array_layer_count: NonZeroU32::new(1),
        });

        Self {
            view,
            width,
            height,
        }
    }

    pub fn from_bytes(gpu: &mut Context, width: u32, height: u32, bytes: &[u8]) -> Self {
        let raw = RawTexture::from_bytes(gpu, width, height, wgpu::TextureUsages::TEXTURE_BINDING, bytes);
        Self::from_raw(&raw, width, height)
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

impl super::AsTextureView for Texture {
    fn as_view(&self) -> &wgpu::TextureView {
        &self.view
    }
}

impl super::AsTextureView for &Texture {
    fn as_view(&self) -> &wgpu::TextureView {
        &self.view
    }
}

#[cfg(feature = "use-image")]
impl Texture {
    pub fn from_image(gpu: &mut Gpu, img: &image::DynamicImage) -> Self {
        let img = img.to_bgra();
        let (width, height) = img.dimensions();
        let bytes = &img.into_raw()[..];
        Self::from_bytes(gpu, width, height, bytes)
    }
}
