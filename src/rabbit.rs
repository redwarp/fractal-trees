use crate::utils::{Bounded, Drawable, Palette};
use skia_safe::{utils::parse_path::from_svg, Canvas, Color, Paint, Path};

const RABBIT_SIZE: f32 = 240.0;
const TINY_RABBIT_SCALE: f32 = 0.2;
const TINY_RABBIT_SIZE: f32 = RABBIT_SIZE * TINY_RABBIT_SCALE;

const RABBIT_SVG: &str = "M122 75V25H125H148H151V75H170V175H70V75H89V25H92H115H118V75H122Z \
    M50 205L70 185V181H67H64V178V175H60L40 195H25V207.5H37.5V220H50V205Z \
    M190 45L170 65V69H173H176V72V75H180L200 55H215V42.5H202.5V30H190V45Z \
    M50 45L70 65V69H67H64V72V75H60L40 55H25V42.5H37.5V30H50V45Z \
    M190 205L170 185V181H173H176V178V175H180L200 195H215V207.5H202.5V220H190V205Z";
const EYES_SVG: &str = "M85 115V135H105V115H85Z \
    M135 135V115H155V135H135Z \
    M100 160V150H140V160H100Z";

struct Rabbits;

enum PatternPosition {
    Full,
    Corner,
    Vertical,
    Horizontal,
}

impl Rabbits {
    fn draw_pattern(
        canvas: &mut Canvas,
        body: &Path,
        eyes: &Path,
        x: i32,
        y: i32,
        paint: &mut Paint,
        pattern_position: PatternPosition,
    ) {
        canvas.save();
        canvas.translate((
            x as f32 * 4.0 * TINY_RABBIT_SIZE,
            y as f32 * 4.0 * TINY_RABBIT_SIZE,
        ));
        let coords: Vec<(f32, f32, Color)> = match pattern_position {
            PatternPosition::Full => vec![
                (0.0, 0.0, Palette::GRAY),
                (0.0, 2.0, Palette::GRAY),
                (2.0, 0.0, Palette::GRAY),
                (2.0, 2.0, Palette::GRAY),
                (1.0, 1.0, Palette::RED),
                (1.0, 3.0, Palette::GRAY),
                (3.0, 1.0, Palette::GRAY),
                (3.0, 3.0, Palette::DARK_BEIGE),
            ],
            PatternPosition::Corner => vec![
                (0.0, 0.0, Palette::GRAY),
                (0.0, 2.0, Palette::GRAY),
                (2.0, 0.0, Palette::GRAY),
                (2.0, 2.0, Palette::GRAY),
                (1.0, 1.0, Palette::RED),
            ],
            PatternPosition::Vertical => vec![
                (0.0, 0.0, Palette::GRAY),
                (0.0, 2.0, Palette::GRAY),
                (2.0, 0.0, Palette::GRAY),
                (2.0, 2.0, Palette::GRAY),
                (1.0, 1.0, Palette::RED),
                (1.0, 3.0, Palette::GRAY),
            ],
            PatternPosition::Horizontal => vec![
                (0.0, 0.0, Palette::GRAY),
                (0.0, 2.0, Palette::GRAY),
                (2.0, 0.0, Palette::GRAY),
                (2.0, 2.0, Palette::GRAY),
                (1.0, 1.0, Palette::RED),
                (3.0, 1.0, Palette::GRAY),
            ],
        };
        for (x, y, color) in &coords {
            Rabbits::draw_rabbit(
                canvas,
                body,
                eyes,
                x * TINY_RABBIT_SIZE,
                y * TINY_RABBIT_SIZE,
                *color,
                paint,
            );
        }
        canvas.restore();
    }

    fn draw_rabbit(
        canvas: &mut Canvas,
        body: &Path,
        eyes: &Path,
        x: f32,
        y: f32,
        color: Color,
        paint: &mut Paint,
    ) {
        canvas.save();

        canvas.translate((x, y));
        paint.set_color(color);
        canvas.draw_path(body, paint);
        paint.set_color(Palette::BEIGE);
        canvas.draw_path(eyes, paint);

        canvas.restore();
    }
}

impl Drawable for Rabbits {
    fn draw(&self, canvas: &mut Canvas) {
        canvas.clear(Palette::BEIGE);

        let body_path = from_svg(RABBIT_SVG);
        let eyes_path = from_svg(EYES_SVG);

        let pattern_count_horizontal = ((canvas.width() / TINY_RABBIT_SIZE).floor() as i32 - 3) / 4;
        let pattern_count_vertical = ((canvas.height() / TINY_RABBIT_SIZE).floor() as i32 - 3) / 4;

        let pattern_margin_top = (canvas.height()
            - (pattern_count_vertical * 4) as f32 * TINY_RABBIT_SIZE
            - TINY_RABBIT_SIZE * 3.0)
            / 2.0;
        let pattern_margin_left = (canvas.width()
            - (pattern_count_horizontal * 4) as f32 * TINY_RABBIT_SIZE
            - TINY_RABBIT_SIZE * 3.0)
            / 2.0;

        if let (Some(body_path), Some(eyes_path)) = (body_path, eyes_path) {
            let mut paint = Paint::default();
            paint.set_anti_alias(true);

            canvas.save();
            canvas.translate((pattern_margin_left, pattern_margin_top));
            let (pattern_body, pattern_eyes) =
                resize_for_pattern(body_path.clone(), eyes_path.clone());

            for x in 0..pattern_count_horizontal + 1 {
                for y in 0..pattern_count_vertical + 1 {
                    let position = match (x, y) {
                        (x, y) if (x, y) == (pattern_count_horizontal, pattern_count_vertical) => {
                            PatternPosition::Corner
                        }
                        (x, _) if x == pattern_count_horizontal => PatternPosition::Vertical,
                        (_, y) if y == pattern_count_vertical => PatternPosition::Horizontal,
                        _ => PatternPosition::Full,
                    };

                    Rabbits::draw_pattern(
                        canvas,
                        &pattern_body,
                        &pattern_eyes,
                        x,
                        y,
                        &mut paint,
                        position,
                    );
                }
            }
            canvas.restore();

            canvas.translate((
                (canvas.width() - RABBIT_SIZE) / 2.0,
                (canvas.height() - RABBIT_SIZE) / 2.0,
            ));

            paint.set_color(Palette::BEIGE);
            canvas.draw_circle(
                (RABBIT_SIZE / 2.0, RABBIT_SIZE / 2.0),
                RABBIT_SIZE * 0.75,
                &paint,
            );
            let mut border_paint = paint.clone();
            border_paint.set_color(Palette::BLACK);
            border_paint.set_style(skia_safe::PaintStyle::Stroke);
            border_paint.set_stroke_width(RABBIT_SIZE * 0.1);
            canvas.draw_circle(
                (RABBIT_SIZE / 2.0, RABBIT_SIZE / 2.0),
                RABBIT_SIZE * 0.75,
                &border_paint,
            );

            paint.set_color(Palette::BLACK);
            canvas.draw_path(&body_path, &Paint::default());

            paint.set_color(Palette::WHITE);
            canvas.draw_path(&eyes_path, &paint);
        }
    }
}

fn resize_for_pattern(mut body: Path, mut eyes: Path) -> (Path, Path) {
    (
        body.make_scale((TINY_RABBIT_SCALE, TINY_RABBIT_SCALE)),
        eyes.make_scale((TINY_RABBIT_SCALE, TINY_RABBIT_SCALE)),
    )
}

pub fn draw(canvas: &mut Canvas) {
    Rabbits.draw(canvas);
}
