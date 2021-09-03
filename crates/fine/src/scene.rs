use crate::frame::Frame;

pub trait Scene {
    fn on_load(frame: Frame) -> Self;
    fn on_start(&mut self) {}
    fn on_event(&mut self) {}
    fn on_draw(&mut self, _frame: Frame) {}
}
