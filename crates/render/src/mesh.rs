pub struct Mesh {
    attributes: Vec<wgpu::Buffer>,
    indices: Option<wgpu::Buffer>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            attributes: Vec::new(),
            indices: None,
        }
    }

    pub fn set_attribute(&mut self, buffer: wgpu::Buffer) {
        self.attributes.push(buffer);
    }

    pub fn set_indices(&mut self, buffer: Option<wgpu::Buffer>) {
        self.indices = buffer;
    }

    pub fn attributes(&self) -> Vec<&wgpu::Buffer> {
        self.attributes.iter().map(|buffer| buffer).collect()
    }

    pub fn indices(&self) -> Option<&wgpu::Buffer> {
        self.indices.as_ref()
    }

    pub fn vertex_count() {

    }

    pub fn create_vertex_buffer(&self) {

    }
}
