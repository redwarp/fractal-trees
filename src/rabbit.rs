use crate::utils::{Bounded, Drawable, Palette};
use skia_safe::{utils::parse_path::from_svg, Canvas, Color, Paint, Path};

const RABBIT_WIDTH: f32 = 240.0;
const RABBIT_HEIGHT: f32 = 240.0;
const TINY_RABBIT_SCALE: f32 = 0.2;
const TINY_RABBIT_WIDTH: f32 = RABBIT_WIDTH * TINY_RABBIT_SCALE;

const RABBIT_SVG: &str = "M122 75V25H125H148H151V75H170V175H70V75H89V25H92H115H118V75H122Z \
    M50 205L70 185V181H67H64V178V175H60L40 195H25V207.5H37.5V220H50V205Z \
    M190 45L170 65V69H173H176V72V75H180L200 55H215V42.5H202.5V30H190V45Z \
    M50 45L70 65V69H67H64V72V75H60L40 55H25V42.5H37.5V30H50V45Z \
    M190 205L170 185V181H173H176V178V175H180L200 195H215V207.5H202.5V220H190V205Z";
const EYES_SVG: &str = "M85 115V135H105V115H85Z \
    M135 135V115H155V135H135Z \
    M100 160V150H140V160H100Z";

struct Rabbits;

impl Rabbits {
    fn draw_pattern(
        canvas: &mut Canvas,
        body: &Path,
        eyes: &Path,
        x: i32,
        y: i32,
        paint: &mut Paint,
    ) {
        canvas.save();
        canvas.translate((
            x as f32 * 4.0 * TINY_RABBIT_WIDTH,
            y as f32 * 4.0 * TINY_RABBIT_WIDTH,
        ));
        let coords: [(f32, f32, Color); 8] = [
            (0.0, 0.0, Palette::BLACK),
            (0.0, 2.0, Palette::BLACK),
            (2.0, 0.0, Palette::BLACK),
            (2.0, 2.0, Palette::BLACK),
            (1.0, 1.0, Palette::RED),
            (1.0, 3.0, Palette::RED),
            (3.0, 1.0, Palette::RED),
            (3.0, 3.0, Palette::RED),
        ];
        for (x, y, color) in &coords {
            Rabbits::draw_rabbit(
                canvas,
                body,
                eyes,
                x * TINY_RABBIT_WIDTH,
                y * TINY_RABBIT_WIDTH,
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

        if let (Some(body_path), Some(eyes_path)) = (body_path, eyes_path) {
            let mut paint = Paint::default();
            paint.set_anti_alias(true);

            let (pattern_body, pattern_eyes) =
                resize_for_pattern(body_path.clone(), eyes_path.clone());

            for x in 0..10 {
                for y in 0..7 {
                    Rabbits::draw_pattern(canvas, &pattern_body, &pattern_eyes, x, y, &mut paint);
                }
            }

            canvas.translate((
                (canvas.width() - RABBIT_WIDTH) / 2.0,
                (canvas.height() - RABBIT_HEIGHT) / 2.0,
            ));

            paint.set_color(Palette::BEIGE);
            canvas.draw_circle(
                (RABBIT_WIDTH / 2.0, RABBIT_HEIGHT / 2.0),
                RABBIT_WIDTH * 0.75,
                &paint,
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
