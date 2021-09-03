#[derive(Debug)]
pub enum VertexValues {
    Float32(Vec<f32>),
    Float32x2(Vec<[f32; 2]>),
    Float32x3(Vec<[f32; 3]>),
    Float32x4(Vec<[f32; 4]>),
}

impl VertexValues {
    pub fn len(&self) -> usize {
        match self {
            VertexValues::Float32(ref values) => values.len(),
            VertexValues::Float32x2(ref values) => values.len(),
            VertexValues::Float32x3(ref values) => values.len(),
            VertexValues::Float32x4(ref values) => values.len(),
        }
    }

    pub fn get_bytes(&self) -> &[u8] {
        match self {
            VertexValues::Float32(ref values) => bytemuck::cast_slice(values),
            VertexValues::Float32x2(ref values) => bytemuck::cast_slice(values),
            VertexValues::Float32x3(ref values) => bytemuck::cast_slice(values),
            VertexValues::Float32x4(ref values) => bytemuck::cast_slice(values),
        }
    }

    pub fn attribute_size(&self) -> u64 {
        self.attribute_format().size()
    }

    pub fn attribute_format(&self) -> wgpu::VertexFormat {
        match self {
            VertexValues::Float32(_) => wgpu::VertexFormat::Float32,
            VertexValues::Float32x2(_) => wgpu::VertexFormat::Float32x2,
            VertexValues::Float32x3(_) => wgpu::VertexFormat::Float32x3,
            VertexValues::Float32x4(_) => wgpu::VertexFormat::Float32x4,
        }
    }
}

impl From<Vec<f32>> for VertexValues {
    fn from(v: Vec<f32>) -> Self {
        Self::Float32(v)
    }
}

impl From<Vec<[f32; 2]>> for VertexValues {
    fn from(v: Vec<[f32; 2]>) -> Self {
        Self::Float32x2(v)
    }
}

impl From<Vec<[f32; 3]>> for VertexValues {
    fn from(v: Vec<[f32; 3]>) -> Self {
        Self::Float32x3(v)
    }
}

impl From<Vec<[f32; 4]>> for VertexValues {
    fn from(v: Vec<[f32; 4]>) -> Self {
        Self::Float32x4(v)
    }
}