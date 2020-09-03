use crate::utils::Bounded;
use skia_safe::{Canvas, Color, Paint};

const SKY_COLOR: Color = Color::WHITE;
#[allow(dead_code)]
const EARTH_COLOR: Color = Color::BLACK;

#[allow(unused_variables)]
pub fn draw(canvas: &mut Canvas) {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);

    let width = canvas.width();
    let height = canvas.height();
    // Fill with the sky color.
    canvas.clear(SKY_COLOR);
}
