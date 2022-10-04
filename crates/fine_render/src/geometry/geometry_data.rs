use crate::prelude::Context;

pub use super::IndiceValues;
pub use super::VertexValues;

#[derive(Debug)]
pub struct GeometryData {
    vertex_values: Vec<VertexValues>,
    index_values: Option<IndiceValues>,
    vertex_size: u64,
}

impl GeometryData {
    pub fn new() -> Self {
        Self {
            vertex_values: Vec::new(),
            index_values: None,
            vertex_size: 0,
        }
    }

    pub fn set_attribute(&mut self, values: VertexValues) {
        self.vertex_size += values.attribute_size();
        self.vertex_values.push(values);
    }

    pub fn set_indices(&mut self, values: Option<IndiceValues>) {
        self.index_values = values;
    }

    #[inline]
    pub fn vertex_size(&self) -> u64 {
        self.vertex_size
    }

    pub fn values(&self) -> Vec<&VertexValues> {
        self.vertex_values.iter().map(|values| values).collect()
    }

    pub fn indices(&self) -> Option<&IndiceValues> {
        self.index_values.as_ref()
    }

    pub fn index_count(&self) -> Option<usize> {
        self.index_values.as_ref().map(|values| values.len())
    }

    pub fn compute_vertex_count(&self) -> usize {
        let mut vertex_count: Option<usize> = None;
        for (index, values) in self.vertex_values.iter().enumerate() {
            let count = values.len();
            if let Some(vertex_count) = vertex_count {
                assert_eq!(
                    vertex_count, count,
                    "Attribute at index {index} does not have the same number of vertex"
                );
            }
            vertex_count = Some(count);
        }
        vertex_count.unwrap_or(0)
    }

    pub fn create_vertex_buffer(&self, cx: &Context) -> wgpu::Buffer {
        let vertex_count = self.compute_vertex_count();
        let vertex_size = self.vertex_size as usize;
        let mut attribute_offset = 0;
        let mut v = vec![0; vertex_size * vertex_count];
        for values in self.vertex_values.iter() {
            let attribute_size = values.attribute_size() as usize;
            let bytes = values.get_bytes();
            for (index, chunk) in bytes.chunks_exact(attribute_size).enumerate() {
                let offset = index * vertex_size + attribute_offset;
                v[(offset)..(offset + attribute_size)].copy_from_slice(chunk);
            }
            attribute_offset += attribute_size;
        }
        cx.create_buffer_with_data(wgpu::BufferUsages::VERTEX, &v)
    }

    pub fn create_index_buffer(&self, cx: &Context) -> Option<wgpu::Buffer> {
        self.index_values.as_ref().map(|indices| {
            cx.create_buffer_with_data(wgpu::BufferUsages::INDEX, indices.get_bytes())
        })
    }

    pub fn create_vertex_attributes(&self) -> Vec<wgpu::VertexAttribute> {
        let locations = self
            .vertex_values
            .iter()
            .enumerate()
            .map(|(location, _)| location as u32);
        self.create_vertex_attributes_with_locations(locations)
    }

    pub fn create_vertex_attributes_with_locations(
        &self,
        locations: impl Iterator<Item = u32>,
    ) -> Vec<wgpu::VertexAttribute> {
        let mut offset = 0;
        self.vertex_values
            .iter()
            .zip(locations)
            .map(|(values, shader_location)| {
                let attribute = wgpu::VertexAttribute {
                    shader_location,
                    format: values.attribute_format(),
                    offset,
                };
                offset += values.attribute_size();
                return attribute;
            })
            .collect::<_>()
    }

    pub fn create_geometry(&self, cx: &super::Context) -> super::Geometry {
        super::Geometry::new(cx, &self)
    }
}
