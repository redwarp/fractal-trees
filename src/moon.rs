use crate::utils::{Bounded, Drawable, Palette};
use skia_safe::Canvas;

struct Night;

impl Drawable for Night {
    fn draw(&self, canvas: &mut skia_safe::Canvas) {
        canvas.clear(Palette::BLACK);
    }
}

pub fn draw(canvas: &mut Canvas) {
    Night.draw(canvas);
}
