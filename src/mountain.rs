use crate::utils::Bounded;
use skia_safe::{Canvas, Color, Paint, Point};
use crate::geometry::*;

const SKY_COLOR: Color = Color::WHITE;
const EARTH_COLOR: Color = Color::BLACK;

#[allow(unused_variables)]
pub fn draw(canvas: &mut Canvas) {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_color(EARTH_COLOR);

    let width = canvas.width();
    let height = canvas.height();

    paint.set_stroke_width(height.min(width)/ 100.0);
    // Fill with the sky color.
    canvas.clear(SKY_COLOR);

    draw_mountain(canvas, Point::new(200.0, 1000.0), 200.0, &paint);

}

fn draw_mountain(canvas: &mut Canvas, origin: Point, width: f32, paint:&Paint){

    let segment = Segment::new(origin.x, origin.y, origin.x + width, origin.y);


    canvas.draw_segment(segment, &paint);
}