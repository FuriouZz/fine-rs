pub enum TextureSource<'a> {
    Buffer(wgpu::ImageCopyBuffer<'a>),
    Texture(wgpu::ImageCopyTexture<'a>),
}
