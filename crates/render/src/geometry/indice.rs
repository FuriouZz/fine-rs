#[derive(Debug)]
pub enum IndiceValues {
    U16(Vec<u16>),
    U32(Vec<u32>),
}

impl IndiceValues {
    pub fn len(&self) -> usize {
        match self {
            IndiceValues::U16(ref values) => values.len(),
            IndiceValues::U32(ref values) => values.len(),
        }
    }

    pub fn get_bytes(&self) -> &[u8] {
        match self {
            IndiceValues::U16(ref values) => bytemuck::cast_slice(values),
            IndiceValues::U32(ref values) => bytemuck::cast_slice(values),
        }
    }

    pub fn index_size(&self) -> usize {
        match self {
            IndiceValues::U16(_) => 2,
            IndiceValues::U32(_) => 4,
        }
    }

    pub fn index_format(&self) -> wgpu::IndexFormat {
        match self {
            IndiceValues::U16(_) => wgpu::IndexFormat::Uint16,
            IndiceValues::U32(_) => wgpu::IndexFormat::Uint32,
        }
    }
}

impl From<Vec<u16>> for IndiceValues {
    fn from(v: Vec<u16>) -> Self {
        Self::U16(v)
    }
}

impl From<Vec<u32>> for IndiceValues {
    fn from(v: Vec<u32>) -> Self {
        Self::U32(v)
    }
}
