use crate::constants::DEFAULT_TEXTURE_FORMAT;

pub struct ContextOptions<'a> {
    /// Power preference
    pub power_preference: wgpu::PowerPreference,
    /// Device options
    pub device: wgpu::DeviceDescriptor<'a>,
    /// Preferred backend
    pub backends: wgpu::Backends,
    /// Swap chain texture usage
    pub usage: wgpu::TextureUsages,
    /// Swap chain texture format
    pub format: wgpu::TextureFormat,
    /// Swap chain present mode
    pub present_mode: wgpu::PresentMode,
}

impl<'a> Default for ContextOptions<'a> {
    fn default() -> Self {
        Self {
            power_preference: wgpu::PowerPreference::default(),
            backends: wgpu::Backends::PRIMARY,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: DEFAULT_TEXTURE_FORMAT,
            present_mode: wgpu::PresentMode::Mailbox,
            device: Default::default(),
        }
    }
}
