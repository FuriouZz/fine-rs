mod indice;
mod vertex;
mod geometry_data;
pub use indice::*;
pub use vertex::*;
pub use geometry_data::*;
use crate::context::Context;

pub struct Geometry {
    pub vertex_buffer: wgpu::Buffer,
    pub vertex_count: usize,
    pub vertex_size: u64,
    pub index_buffer: Option<wgpu::Buffer>,
    pub index_count: Option<usize>,
    pub index_format: Option<wgpu::IndexFormat>,
}

impl Geometry {
    pub fn new(context: &Context, geometry: &GeometryData) -> Self {
        let vertex_buffer = geometry.create_vertex_buffer(context);
        let vertex_count = geometry.compute_vertex_count();
        let vertex_size = geometry.vertex_size();

        let index_buffer = geometry.create_index_buffer(context);
        let index_count = geometry.indices().map(|i| i.len());
        let index_format = geometry.indices().map(|i| i.index_format());

        Self {
            vertex_buffer,
            vertex_count,
            vertex_size,
            index_buffer,
            index_count,
            index_format,
        }
    }

    pub fn bind<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>, slot: u32) {
        pass.set_vertex_buffer(slot, self.vertex_buffer.slice(..));
    }

    pub fn draw<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        match (
            self.index_buffer.as_ref(),
            self.index_count.as_ref(),
            self.index_format.as_ref(),
        ) {
            (Some(index_buffer), Some(index_count), Some(index_format)) => {
                pass.set_index_buffer(index_buffer.slice(..), *index_format);
                pass.draw_indexed(0..(*index_count as u32), 0, 0..1);
            }
            _ => {
                pass.draw(0..(self.vertex_count as u32), 0..1);
            }
        }
    }
}
