mod context_types;
mod options;
mod surface;
pub use context_types::*;
pub use options::*;
use std::{borrow::Cow, rc::Rc};
pub use surface::*;
use wgpu::util::DeviceExt;

use crate::pipeline::PipelineBuilder;

pub struct Context {
    device: Rc<wgpu::Device>,
    encoder: wgpu::CommandEncoder,
    queue: wgpu::Queue,
}

impl Context {
    pub async fn new<'a, W>(
        window: &W,
        options: &ContextOptions<'a>,
    ) -> Option<(Context, surface::Surface)>
    where
        W: raw_window_handle::HasRawWindowHandle,
    {
        let instance = wgpu::Instance::new(options.backends);

        // Create a surface to draw
        let surface = unsafe { instance.create_surface(window) };

        // Request the more appropriate adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: options.power_preference,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

        let (device, queue) = adapter.request_device(&options.device, None).await.ok()?;

        let surface_configuration = wgpu::SurfaceConfiguration {
            usage: options.usage,
            format: options.format,
            width: 800,
            height: 600,
            present_mode: options.present_mode,
        };

        surface.configure(&device, &surface_configuration);

        let encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        let context = Context {
            device: Rc::new(device),
            queue,
            encoder,
        };

        let surface = surface::Surface {
            surface,
            configuration: surface_configuration,
            output: None,
        };

        Some((context, surface))
    }

    pub fn create_wgsl_shader(&self, source: &str) -> wgpu::ShaderModule {
        self.device
            .create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: Some("[fine_render] Create shader module"),
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(source)),
            })
    }

    pub fn create_buffer(
        &self,
        mapped_at_creation: bool,
        usage: wgpu::BufferUsages,
        size: u64,
    ) -> wgpu::Buffer {
        self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Context::create_buffer"),
            mapped_at_creation,
            usage,
            size,
        })
    }

    pub fn create_buffer_with_data(&self, usage: wgpu::BufferUsages, bytes: &[u8]) -> wgpu::Buffer {
        self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Context::create_buffer_with_data"),
                usage,
                contents: bytes,
            })
    }

    pub fn create_texture(&self, desc: &wgpu::TextureDescriptor) -> wgpu::Texture {
        self.device.create_texture(desc)
    }

    pub fn create_pipeline(&self) -> PipelineBuilder {
        PipelineBuilder::new(self.device.clone())
    }

    pub fn write_buffer(&mut self, destination: &wgpu::Buffer, data: &[u8]) {
        let source = self.create_buffer_with_data(wgpu::BufferUsages::COPY_SRC, data);
        self.encoder
            .copy_buffer_to_buffer(&source, 0, destination, 0, data.len() as u64);
    }

    pub fn write_texture(
        &mut self,
        source: TextureSource,
        destination: wgpu::ImageCopyTexture,
        copy_size: wgpu::Extent3d,
    ) {
        match source {
            TextureSource::Buffer(buffer) => {
                self.encoder
                    .copy_buffer_to_texture(buffer, destination, copy_size)
            }
            TextureSource::Texture(texture) => {
                self.encoder
                    .copy_texture_to_texture(texture, destination, copy_size)
            }
        }
    }

    pub fn enqueue_write_buffer(
        &self,
        buffer: &wgpu::Buffer,
        offset: wgpu::BufferAddress,
        data: &[u8],
    ) {
        self.queue.write_buffer(&buffer, offset, data);
    }

    pub fn enqueue_write_texture(
        &self,
        texture: wgpu::ImageCopyTexture,
        data: &[u8],
        data_layout: wgpu::ImageDataLayout,
        size: wgpu::Extent3d,
    ) {
        self.queue.write_texture(texture, data, data_layout, size);
    }

    pub fn begin_render_pass<'a>(
        &'a mut self,
        desc: &wgpu::RenderPassDescriptor<'a, '_>,
    ) -> wgpu::RenderPass<'a> {
        self.encoder.begin_render_pass(desc)
    }

    pub fn begin_compute_pass(&mut self, desc: &wgpu::ComputePassDescriptor) -> wgpu::ComputePass {
        self.encoder.begin_compute_pass(desc)
    }

    pub fn submit_queue(&mut self, surface: &mut Surface) {
        let encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        let current = std::mem::replace(&mut self.encoder, encoder);
        self.queue.submit(Some(current.finish()));
        surface.output = None;
    }
}
