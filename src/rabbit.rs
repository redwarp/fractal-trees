use crate::utils::{Bounded, Drawable, Palette};
use skia_safe::{utils::parse_path::from_svg, Canvas, Color, Paint, Path, Rect};

const RABBIT_SIZE: f32 = 240.0;
const TINY_RABBIT_SIZE: f32 = 40.0;
const STROKE_WIDTH: f32 = 4.0;
const BORDER_SIZE: f32 = 80.0;

const RABBIT_SVG: &str = "M122 75V25H125H148H151V75H170V175H70V75H89V25H92H115H118V75H122Z \
    M50 205L70 185V181H67H64V178V175H60L40 195H25V207.5H37.5V220H50V205Z \
    M190 45L170 65V69H173H176V72V75H180L200 55H215V42.5H202.5V30H190V45Z \
    M50 45L70 65V69H67H64V72V75H60L40 55H25V42.5H37.5V30H50V45Z \
    M190 205L170 185V181H173H176V178V175H180L200 195H215V207.5H202.5V220H190V205Z";
const EYES_SVG: &str = "M85 115V135H105V115H85Z \
    M135 135V115H155V135H135Z \
    M100 160V150H140V160H100Z";
const TINY_RABBIT_SVG: &str = "M21 13V4H22H24H25V13H28V29H12V13H15V4H16.5H17.5H19V13H21Z \
    M8.5 34L12 30.5V30H11.3333H11V29.5V29H10.5L7 32.5H4.5V34.5H6.5V36.5H8.5V34Z \
    M31.5 34L28 30.5V30H28.6667H29V29.5V29H29.5L33 32.5H35.5V34.5H33.5V36.5H31.5V34Z \
    M31.5 8L28 11.5V12H28.6667H29V12.5V13H29.5L33 9.50001H35.5V7.5H33.5V5.5H31.5V8Z \
    M8.5 8L12 11.5V12H11.3333H11V12.5V13H10.5L7 9.50001H4.5V7.5H6.5V5.5H8.5V8Z";
const TINY_EYES_SVG: &str = "M14 19V22.5H17.5V19H14Z
    M22.5 22.5V19L26 19V22.5L22.5 22.5Z
    M16.5 26.5V25H23.5V26.5H20H16.5Z";

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

    fn draw_border(canvas: &mut Canvas, paint: &mut Paint, border_paint: &mut Paint) {
        paint.set_color(Palette::BEIGE);
        canvas.draw_rect(Rect::new(0.0, 0.0, canvas.width(), BORDER_SIZE), &paint);
        canvas.draw_rect(
            Rect::new(
                0.0,
                canvas.height() - BORDER_SIZE,
                canvas.width(),
                canvas.height(),
            ),
            &paint,
        );
        canvas.draw_rect(Rect::new(0.0, 0.0, BORDER_SIZE, canvas.height()), &paint);
        canvas.draw_rect(
            Rect::new(
                canvas.width() - BORDER_SIZE,
                0.0,
                canvas.width(),
                canvas.height(),
            ),
            &paint,
        );

        let mut border_path = Path::new();
        border_path.move_to((BORDER_SIZE, BORDER_SIZE));
        border_path.line_to((canvas.width() - BORDER_SIZE, BORDER_SIZE));
        border_path.line_to((canvas.width() - BORDER_SIZE, canvas.height() - BORDER_SIZE));
        border_path.line_to((BORDER_SIZE, canvas.height() - BORDER_SIZE));
        border_path.close();
        canvas.draw_path(&border_path, &border_paint);
    }
}

impl Drawable for Rabbits {
    fn draw(&self, canvas: &mut Canvas) {
        canvas.clear(Palette::BEIGE);

        let body_path = from_svg(RABBIT_SVG);
        let eyes_path = from_svg(EYES_SVG);
        let tiny_body_path = from_svg(TINY_RABBIT_SVG);
        let tiny_eyes_path = from_svg(TINY_EYES_SVG);

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

        let mut paint = Paint::default();
        paint.set_anti_alias(true);

        let mut border_paint = Paint::default();
        border_paint.set_anti_alias(true);
        border_paint.set_color(Palette::BLACK);
        border_paint.set_style(skia_safe::PaintStyle::Stroke);
        border_paint.set_stroke_width(STROKE_WIDTH);

        if let (Some(body_path), Some(eyes_path), Some(tiny_body_path), Some(tiny_eyes_path)) =
            (body_path, eyes_path, tiny_body_path, tiny_eyes_path)
        {
            canvas.save();
            canvas.translate((pattern_margin_left, pattern_margin_top));

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
                        &tiny_body_path,
                        &tiny_eyes_path,
                        x,
                        y,
                        &mut paint,
                        position,
                    );
                }
            }
            canvas.restore();

            canvas.save();
            canvas.translate((
                (canvas.width() - RABBIT_SIZE) / 2.0,
                (canvas.height() - RABBIT_SIZE) / 2.0,
            ));

            paint.set_color(Palette::BEIGE);
            canvas.draw_circle(
                (RABBIT_SIZE / 2.0, RABBIT_SIZE / 2.0),
                RABBIT_SIZE * 0.8,
                &paint,
            );
            canvas.draw_circle(
                (RABBIT_SIZE / 2.0, RABBIT_SIZE / 2.0),
                RABBIT_SIZE * 0.8,
                &border_paint,
            );

            paint.set_color(Palette::BLACK);
            canvas.draw_path(&body_path, &Paint::default());

            paint.set_color(Palette::WHITE);
            canvas.draw_path(&eyes_path, &paint);

            canvas.restore();
        }

        Rabbits::draw_border(canvas, &mut paint, &mut border_paint);
    }
}

pub fn draw(canvas: &mut Canvas) {
    Rabbits.draw(canvas);
}
