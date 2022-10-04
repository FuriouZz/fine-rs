use crate::{context::Context, prelude::DEFAULT_TEXTURE_FORMAT};
use std::num::NonZeroU32;

pub struct RawTexture {
    raw: wgpu::Texture,
    width: u32,
    height: u32,
}

impl RawTexture {
    pub fn new(raw: wgpu::Texture, width: u32, height: u32) -> Self {
        Self { raw, width, height }
    }

    pub fn from_bytes(
        gpu: &mut Context,
        width: u32,
        height: u32,
        usage: wgpu::TextureUsages,
        bytes: &[u8],
    ) -> Self {
        // Create texture
        let raw = gpu.create_texture(&wgpu::TextureDescriptor {
            label: Some("RawTexture::from_bytes"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: DEFAULT_TEXTURE_FORMAT,
            usage: wgpu::TextureUsages::COPY_DST | usage,
        });

        gpu.enqueue_write_texture(
            wgpu::ImageCopyTexture {
                texture: &raw,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            bytes,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(4 * width),
                rows_per_image: NonZeroU32::new(0),
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );

        Self::new(raw, width, height)
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

    pub fn as_raw(&self) -> &wgpu::Texture {
        &self.raw
    }
}

#[cfg(feature = "use-image")]
impl Raw {
    pub fn from_image(gpu: &mut Gpu, usage: wgpu::TextureUsage, img: &image::DynamicImage) -> Self {
        let img = img.to_bgra();
        let (width, height) = img.dimensions();
        let bytes = &img.into_raw()[..];
        Self::from_bytes(gpu, width, height, usage, bytes)
    }
}
