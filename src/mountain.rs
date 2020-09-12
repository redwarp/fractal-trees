use crate::geometry::*;
use crate::utils::Bounded;
use skia_safe::{Canvas, Color, Paint, PaintStyle, Path, Point};

const SKY_COLOR: Color = Color::new(0xfffceccb);
const EARTH_COLOR: Color = Color::BLACK;

pub fn draw(canvas: &mut Canvas) {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);

    let width = canvas.width();
    let height = canvas.height();

    paint.set_style(PaintStyle::Fill);
    paint.set_stroke_width(height.min(width) / 100.0);
    // Fill with the sky color.
    canvas.clear(SKY_COLOR);

    let base = Segment::new(200.0, 1000.0, 800.0, 1000.0);
    let summit = base
        .point_at_position(0.45)
        .move_along(base.normal(), -base.length() * 0.8);
    draw_mountain(canvas, base, summit, EARTH_COLOR, &mut paint, 0);
}

fn draw_mountain(
    canvas: &mut Canvas,
    base: Segment,
    summit: Point,
    color: Color,
    paint: &mut Paint,
    count: u32,
) {
    paint.set_color(color);

    // canvas.draw_segment(segment, &paint);

    let mut path: Path = Path::new();
    path.move_to(base.a());
    path.line_to(base.b());
    println!("Segment: {:?}", base);

    path.line_to(summit);
    path.close();

    canvas.draw_path(&path, paint);

    if count < 2 {
        let color = if count % 2 == 0 {
            Color::WHITE
        } else {
            Color::BLACK
        };

        let side_a = Segment::from_points(base.a(), summit).point_at_position(0.60);
        let side_b = Segment::from_points(base.b(), summit).point_at_position(0.60);
        draw_mountain(
            canvas,
            Segment::from_points(summit, side_a),
            side_b,
            color,
            paint,
            count + 1,
        );
    };
}
