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
    pub format: Option<wgpu::TextureFormat>,
    /// Swap chain present mode
    pub present_mode: wgpu::PresentMode,
}

impl<'a> Default for ContextOptions<'a> {
    fn default() -> Self {
        Self {
            power_preference: wgpu::PowerPreference::default(),
            backends: wgpu::Backends::all(),
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: None,
            present_mode: wgpu::PresentMode::Mailbox,
            device: wgpu::DeviceDescriptor::default(),
        }
    }
}
