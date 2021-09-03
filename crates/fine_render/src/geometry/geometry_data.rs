use crate::prelude::Context;
use std::collections::HashMap;

pub use super::IndiceValues;
pub use super::VertexValues;

#[derive(Debug)]
pub struct GeometryData {
    vertex_values: HashMap<&'static str, VertexValues>,
    index_values: Option<IndiceValues>,
    vertex_size: u64,
}

impl GeometryData {
    pub fn new() -> Self {
        Self {
            vertex_values: HashMap::new(),
            index_values: None,
            vertex_size: 0,
        }
    }

    pub fn set_attribute(&mut self, name: &'static str, values: VertexValues) {
        self.vertex_size += values.attribute_size();

        self.vertex_values.insert(name, values);
    }

    pub fn set_indices(&mut self, values: Option<IndiceValues>) {
        self.index_values = values;
    }

    pub fn vertex_size(&self) -> u64 {
        self.vertex_size
    }

    pub fn values(&self) -> Vec<(&'static str, &VertexValues)> {
        self.vertex_values
            .iter()
            .map(|(name, values)| (*name, values))
            .collect()
    }

    pub fn indices(&self) -> Option<&IndiceValues> {
        self.index_values.as_ref()
    }

    pub fn index_count(&self) -> Option<usize> {
        self.index_values.as_ref().map(|values| values.len())
    }

    pub fn compute_vertex_count(&self) -> usize {
        let mut vertex_count: Option<usize> = None;
        for (name, values) in self.vertex_values.iter() {
            let count = values.len();
            if let Some(vertex_count) = vertex_count {
                assert_eq!(
                    vertex_count, count,
                    "Attribute {} does not have the same number of vertex",
                    name
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
        for values in self.vertex_values.values() {
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
        let mut offset = 0;
        let mut v = Vec::new();
        for values in self.vertex_values.values() {
            let attribute = wgpu::VertexAttribute {
                shader_location: v.len() as u32,
                format: values.attribute_format(),
                offset,
            };
            v.push(attribute);
            offset += values.attribute_size();
        }
        v
    }
}
