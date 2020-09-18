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

    let base = Segment::new(1100.0, canvas.height(), 1600.0, canvas.height());
    let summit = base
        .point_at_position(0.45)
        .move_along(base.normal(), -base.length() * 0.9);
    draw_mountain(
        canvas,
        base,
        summit,
        Color::new(0xff333333),
        Color::WHITE,
        &mut paint,
        0,
    );

    let base = Segment::new(700.0, canvas.height(), 1350.0, canvas.height());
    let summit = base
        .point_at_position(0.45)
        .move_along(base.normal(), -base.length() * 0.9);
    draw_mountain(
        canvas,
        base,
        summit,
        Color::new(0xff191919),
        Color::WHITE,
        &mut paint,
        0,
    );

    let base = Segment::new(200.0, canvas.height(), 1000.0, canvas.height());
    let summit = base
        .point_at_position(0.45)
        .move_along(base.normal(), -base.length() * 0.9);
    draw_mountain(
        canvas,
        base,
        summit,
        EARTH_COLOR,
        Color::WHITE,
        &mut paint,
        0,
    );

    let sun_position = (canvas.width() - 350.0, 350.0);
    paint.set_color(Color::RED);
    canvas.draw_circle(sun_position, 125.0, &paint);
}

fn draw_mountain(
    canvas: &mut Canvas,
    base: Segment,
    summit: Point,
    dark_color: Color,
    light_color: Color,
    paint: &mut Paint,
    count: u32,
) {
    let color = if count % 2 == 1 {
        light_color
    } else {
        dark_color
    };
    paint.set_color(color);

    let mut path: Path = Path::new();
    path.move_to(base.a());
    path.line_to(base.b());
    println!("Segment: {:?}", base);

    path.line_to(summit);
    path.close();

    canvas.draw_path(&path, paint);

    if count < 2 {
        let side_a = Segment::from_points(base.a(), summit).point_at_position(0.60);
        let side_b = Segment::from_points(base.b(), summit).point_at_position(0.60);
        draw_mountain(
            canvas,
            Segment::from_points(summit, side_a),
            side_b,
            dark_color,
            light_color,
            paint,
            count + 1,
        );
    };
}
