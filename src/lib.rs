use std::{time::Duration, vec, slice::IterMut};

pub mod vulnus;
pub mod soundspace;



pub trait DefaultMapNote {
    fn set_offset(&mut self, offset: Duration) {
        *self.time_as_mut() += (offset.as_millis() as f64) / 1000f64;
    }
    fn resize(&mut self, by: i32) {
        *self.x_as_mut() = self.x_as_ref().powf(by as f32);
        *self.y_as_mut() = self.y_as_ref().powf(by as f32);
    }

    fn time_as_mut(&mut self) -> &mut f64;
    fn x_as_mut(&mut self) -> &mut f32;
    fn y_as_mut(&mut self) -> &mut f32;
    fn time_as_ref(&self) -> &f64;
    fn x_as_ref(&self) -> &f32;
    fn y_as_ref(&self) -> &f32;

}