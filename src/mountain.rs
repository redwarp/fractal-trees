use crate::geometry::*;
use crate::utils::Bounded;
use skia_safe::{Canvas, Color, Paint, PaintStyle, Path, Point};

const SKY_COLOR: Color = Color::WHITE;
const EARTH_COLOR: Color = Color::BLACK;

pub fn draw(canvas: &mut Canvas) {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_color(EARTH_COLOR);

    let width = canvas.width();
    let height = canvas.height();

    paint.set_style(PaintStyle::Fill);
    paint.set_stroke_width(height.min(width) / 100.0);
    // Fill with the sky color.
    canvas.clear(SKY_COLOR);

    draw_mountain(canvas, Point::new(200.0, 1000.0), 200.0, &paint);
}

fn draw_mountain(canvas: &mut Canvas, origin: Point, width: f32, paint: &Paint) {
    let segment = Segment::new(origin.x, origin.y, origin.x + width, origin.y);

    // canvas.draw_segment(segment, &paint);

    let mut path: Path = Path::new();
    path.move_to(segment.a());
    path.line_to(segment.b());
    println!("Segment: {:?}", segment);

    let summit = segment
        .point_at_position(0.45)
        .move_along(segment.normal(), -segment.length() * 1.2);
    println!("Summit: {:?}", summit);

    path.line_to(summit);
    path.close();

    canvas.draw_path(&path, paint);
}
