use crate::utils::{Bounded, Drawable, Palette};
use skia_safe::{Canvas, Color, Paint};

use rand::{rngs::StdRng, Rng, SeedableRng};

type Position = (f32, f32);

struct Star {
    position: Position,
    color: Color,
    radius: f32,
}

struct Night {
    stars: Vec<Star>,
}

impl Night {
    fn new(star_count: usize, mut rng: StdRng) -> Self {
        let mut stars: Vec<Star> = Vec::with_capacity(star_count);
        for _ in 0..star_count {
            stars.push(Night::random_star(&mut rng));
        }

        Night { stars }
    }

    fn random_star(rng: &mut StdRng) -> Star {
        const COLORS: [Color; 5] = [
            Palette::WHITE,
            Palette::DARK_GRAY,
            Palette::LIGHT_GRAY,
            Palette::BEIGE,
            Palette::RED,
        ];
        Star {
            position: (rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0)),
            color: COLORS[rng.gen_range(0, COLORS.len())],
            radius: rng.gen_range(0.5, 1.0),
        }
    }
}

impl Drawable for Night {
    fn draw(&self, canvas: &mut skia_safe::Canvas) {
        canvas.clear(Palette::BLACK);
        let width = canvas.width();
        let height = canvas.height();
        let star_scale = 5.0;
        let mut star_paint = Paint::default();
        star_paint.set_anti_alias(true);

        for star in self.stars.iter() {
            star_paint.set_color(star.color);
            canvas.draw_circle(
                (star.position.0 * width, star.position.1 * height),
                star.radius * star_scale,
                &star_paint,
            );
        }
    }
}

pub fn draw(canvas: &mut Canvas) {
    let rng = StdRng::seed_from_u64(42);
    let star_count = (canvas.width() * canvas.height() * 0.0001) as usize;

    Night::new(star_count, rng).draw(canvas);
}
