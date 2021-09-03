use super::{RawTexture, Texture};
use crate::context::Context;
use crate::prelude::TextureSource;
use std::collections::HashMap;
use std::num::NonZeroU32;

#[derive(Clone, Debug)]
pub struct TextureRect {
    width: u32,
    height: u32,
    x: u32,
    y: u32,
}

pub type TexturePosition = (u32, u32);

pub struct TextureAtlas {
    layer_count: u32,
    width: u32,
    height: u32,
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    rectangles: HashMap<String, (u32, TextureRect)>,
}

impl TextureAtlas {
    #[allow(dead_code)]
    const MAX_LAYER_COUNT: usize = 256;

    pub fn new(gpu: &mut Context, width: u32, height: u32, layer_count: u32) -> Self {
        let texture = gpu.create_texture(&wgpu::TextureDescriptor {
            label: Some("TextureAtlas::new"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: layer_count,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8Unorm,
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::COPY_DST,
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: None,
            format: Some(wgpu::TextureFormat::Bgra8Unorm),
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            aspect: wgpu::TextureAspect::default(),
            base_mip_level: 0,
            mip_level_count: NonZeroU32::new(1),
            base_array_layer: 0,
            array_layer_count: NonZeroU32::new(layer_count),
        });

        Self {
            layer_count,
            width,
            height,
            texture,
            view,
            rectangles: HashMap::new(),
        }
    }

    pub fn frame(&self, name: impl Into<String>) -> Option<(u32, TextureRect)> {
        self.rectangles.get(&name.into()).map(|r| r.clone())
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn layer_count(&self) -> u32 {
        self.layer_count
    }

    pub fn dimensions(&self) -> (u32, u32, u32) {
        (self.width, self.height, self.layer_count)
    }

    pub fn append_raw_texture<S>(
        &mut self,
        name: S,
        layer: u32,
        gpu: &mut Context,
        texture: &RawTexture,
        source: TextureRect,
        destination: Option<TexturePosition>,
    ) where
        S: Into<String>,
    {
        self.assert(source.width, source.height, layer);

        let origin = if let Some(origin) = destination {
            wgpu::Origin3d {
                x: origin.0,
                y: origin.1,
                z: 0,
            }
        } else {
            wgpu::Origin3d::ZERO
        };

        gpu.write_texture(
            TextureSource::Texture(wgpu::ImageCopyTexture {
                texture: texture.as_raw(),
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: source.x,
                    y: source.y,
                    z: 0,
                },
                aspect: wgpu::TextureAspect::All,
            }),
            wgpu::ImageCopyTexture {
                texture: &self.texture,
                mip_level: layer,
                origin,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::Extent3d {
                width: source.width,
                height: source.height,
                depth_or_array_layers: 1,
            },
        );

        let _ = self.rectangles.insert(
            name.into(),
            (
                layer,
                TextureRect {
                    width: source.width,
                    height: source.height,
                    x: origin.x,
                    y: origin.y,
                },
            ),
        );
    }

    pub fn append_bytes<F>(
        &mut self,
        gpu: &mut Context,
        bytes: &[u8],
        width: u32,
        height: u32,
        mut f: F,
    ) where
        F: FnMut(&mut TextureAtlasFromBytes),
    {
        let texture =
            &RawTexture::from_bytes(gpu, width, height, wgpu::TextureUsages::COPY_SRC, bytes);

        f(&mut TextureAtlasFromBytes {
            gpu,
            texture,
            atlas: self,
        });
    }

    pub fn frame_to_raw(&self, gpu: &mut Context, name: impl Into<String>) -> Option<RawTexture> {
        self.frame(name).map(|frame| {
            let width = frame.1.width;
            let height = frame.1.height;

            let raw = gpu.create_texture(&wgpu::TextureDescriptor {
                label: Some("[fine::graphic::TextureAtlas] frame_to_raw_texture"),
                format: wgpu::TextureFormat::Bgra8Unorm,
                dimension: wgpu::TextureDimension::D2,
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                usage: wgpu::TextureUsages::COPY_DST,
            });

            gpu.write_texture(
                TextureSource::Texture(wgpu::ImageCopyTexture {
                    texture: &self.texture,
                    mip_level: frame.0,
                    origin: wgpu::Origin3d {
                        x: frame.1.x,
                        y: frame.1.y,
                        z: 0,
                    },
                    aspect: wgpu::TextureAspect::All,
                }),
                wgpu::ImageCopyTexture {
                    texture: &raw,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
            );

            RawTexture::new(raw, width, height)
        })
    }

    pub fn frame_to_texture(&self, gpu: &mut Context, name: impl Into<String>) -> Option<Texture> {
        self.frame_to_raw(gpu, name).map(|raw| {
            let width = raw.width();
            let height = raw.height();
            Texture::from_raw(&raw, width, height)
        })
    }

    fn assert(&self, width: u32, height: u32, layer: u32) {
        assert!(
            layer <= self.layer_count,
            "[fine::graphic::TextureAtlas] Cannot add more layers."
        );
        assert!(
            width <= self.width,
            "[fine::graphic::TextureAtlas] width is bigger than the atlas width."
        );
        assert!(
            height <= self.height,
            "[fine::graphic::TextureAtlas] height is bigger than the atlas height."
        );
    }
}

impl super::AsTextureView for TextureAtlas {
    fn as_view(&self) -> &wgpu::TextureView {
        &self.view
    }
}

impl super::AsTextureView for &TextureAtlas {
    fn as_view(&self) -> &wgpu::TextureView {
        &self.view
    }
}

pub struct TextureAtlasFromBytes<'a> {
    gpu: &'a mut Context,
    texture: &'a RawTexture,
    atlas: &'a mut TextureAtlas,
}

impl<'a> TextureAtlasFromBytes<'a> {
    pub fn add<S>(
        &mut self,
        name: S,
        layer: u32,
        source: TextureRect,
        destination: Option<TexturePosition>,
    ) -> &Self
    where
        S: Into<String>,
    {
        self.atlas
            .append_raw_texture(name, layer, self.gpu, &self.texture, source, destination);
        self
    }
}

#[cfg(feature = "use-image")]
impl TextureAtlas {
    pub fn append_image<F>(&mut self, gpu: &mut Gpu, img: &image::DynamicImage, f: F)
    where
        F: FnMut(&mut TextureAtlasFromBytes),
    {
        let img = img.to_bgra();
        let (width, height) = img.dimensions();
        let bytes = &img.into_raw()[..];
        self.append_bytes(gpu, bytes, width, height, f);
    }
}
