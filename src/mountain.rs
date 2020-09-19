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

    let base_origin = 0.57 * width;
    let base_length = 0.26 * width;
    let base = Segment::new(
        base_origin,
        canvas.height(),
        base_origin + base_length,
        canvas.height(),
    );
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
    );

    let base_origin = 0.36 * width;
    let base_length = 0.34 * width;
    let base = Segment::new(
        base_origin,
        canvas.height(),
        base_origin + base_length,
        canvas.height(),
    );
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
    );

    let base_origin = 0.1 * width;
    let base_length = 0.4 * width;
    let base = Segment::new(
        base_origin,
        canvas.height(),
        base_origin + base_length,
        canvas.height(),
    );
    let summit = base
        .point_at_position(0.45)
        .move_along(base.normal(), -base.length() * 0.9);
    draw_mountain(canvas, base, summit, EARTH_COLOR, Color::WHITE, &mut paint);

    let sun_scale = width.min(height);
    let sun_position = (canvas.width() - sun_scale * 0.32, sun_scale * 0.32);
    paint.set_color(Color::RED);
    canvas.draw_circle(sun_position, sun_scale * 0.115, &paint);
}

fn draw_mountain(
    canvas: &mut Canvas,
    base: Segment,
    summit: Point,
    dark_color: Color,
    light_color: Color,
    paint: &mut Paint,
) {
    paint.set_color(dark_color);

    let mut path: Path = Path::new();
    path.move_to(base.a());
    path.line_to(base.b());
    println!("Segment: {:?}", base);

    path.line_to(summit);
    path.close();

    canvas.draw_path(&path, paint);

    {
        let snow_start = Segment::from_points(base.a(), summit).point_at_position(0.60);
        let side = Segment::from_points(base.b(), summit).point_at_position(0.60);

        let mut path = Path::new();
        path.move_to(snow_start);
        path.line_to(summit);
        path.line_to(Segment::from_points(summit, side).point_at_position(0.60));
        path.line_to(Segment::from_points(snow_start, side).point_at_position(0.60));
        path.close();

        paint.set_color(light_color);
        canvas.draw_path(&path, paint);
    }
}
