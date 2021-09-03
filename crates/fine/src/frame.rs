use crate::render;
use crate::render::prelude::wgpu;

pub struct Frame<'a> {
  pub(crate) context: &'a mut render::context::Context,
  pub(crate) surface: &'a mut render::context::Surface,
}

impl<'a> Frame<'a> {
  pub fn width(&self) -> u32 {
      self.surface.width()
  }

  pub fn height(&self) -> u32 {
    self.surface.height()
  }

  pub fn dimensions(&self) -> (u32, u32) {
      (self.surface.width(), self.surface.height())
  }

  pub fn get_surface(&self) -> &render::context::Surface {
    self.surface
  }

  pub fn get_context(&self) -> &render::context::Context {
      self.context
  }

  pub fn get_mut_context(&mut self) -> &mut render::context::Context {
      self.context
  }

  pub fn get_attachment(&mut self) -> (&mut render::context::Context, &wgpu::Texture) {
      (&mut self.context, self.surface.get_current_frame())
  }
}
