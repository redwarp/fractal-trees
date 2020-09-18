use skia_safe::{Canvas, Color, Paint, PaintStyle, Path};

use crate::utils::Bounded;

const BACKGROUND_COLOR: Color = Color::new(0xfffceccb);
const PRECISION: i32 = 128;
const SCALE: f32 = 0.5;

pub fn draw(canvas: &mut Canvas) {
    canvas.clear(BACKGROUND_COLOR);

    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_style(PaintStyle::Stroke);
    paint.set_stroke_width(0.1);

    let mut path = Path::new();
    path.move_to(get_coords(-std::f32::consts::PI));

    for step in (-PRECISION + 1)..PRECISION {
        let step = step as f32 / PRECISION as f32 * std::f32::consts::PI;
        path.line_to(get_coords(step));
    }
    path.close();

    canvas.save();

    let center = (canvas.width() / 2.0, canvas.height() / 2.0);
    canvas.translate(center);
    let scale = canvas.width().min(canvas.height()) * SCALE;
    canvas.scale((scale, scale));

    canvas.save();
    canvas.translate((0.01, 0.01));
    paint.set_color(Color::RED);
    canvas.draw_path(&path, &paint);
    canvas.restore();

    paint.set_color(Color::BLACK);
    canvas.draw_path(&path, &paint);

    canvas.restore();
}

/// Get the coordinate of a point of the symbol, following a formula found
/// here: https://gamedev.stackexchange.com/a/43704/143738
fn get_coords(step: f32) -> (f32, f32) {
    let scale = 2.0 / (3.0 - (2.0_f32 * step).cos());
    let x = scale * step.cos();
    let y = scale * (2.0_f32 * step).sin() / 2.0;

    (x, y)
}
