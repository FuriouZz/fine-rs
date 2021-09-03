use crate::context::Context;

pub struct Surface {
    pub(crate) surface: wgpu::Surface,
    pub(crate) configuration: wgpu::SurfaceConfiguration,
    pub(crate) output: Option<wgpu::SurfaceFrame>,
}

impl Surface {
    pub fn width(&self) -> u32 {
        self.configuration.width
    }

    pub fn height(&self) -> u32 {
        self.configuration.height
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.configuration.width, self.configuration.height)
    }

    pub fn format(&self) -> wgpu::TextureFormat {
        self.configuration.format
    }

    pub fn get_current_frame(&mut self) -> &wgpu::Texture {
        if self.output.is_none() {
            let frame = self
                .surface
                .get_current_frame()
                .expect("Timeout when acquiring next swap chain texture");
            self.output = Some(frame);
        }
        &self.output.as_ref().unwrap().output.texture
    }

    pub fn resize(&mut self, context: &Context, width: u32, height: u32) {
        self.configuration.width = width;
        self.configuration.height = height;
        self.surface.configure(&context.device, &self.configuration);
        self.output = None;
    }
}
