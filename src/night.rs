use crate::utils::{Bounded, Drawable, Palette};
use rand::{rngs::StdRng, Rng, SeedableRng};
use skia_safe::{Canvas, Color, Paint};

type Position = (f32, f32);

const SPOT_COUNT: usize = 80;
const SPOT_SCALE: f32 = 0.25;

struct Star {
    position: Position,
    color: Color,
    radius: f32,
}

/// Use https://stackoverflow.com/questions/5837572/generate-a-random-point-within-a-circle-uniformly
/// to design spots.
struct Moon {
    spots: Vec<Spot>,
}

struct Spot {
    placement: f32,
    angle: f32,
    radius: f32,
    color: Color,
}

struct Night {
    stars: Vec<Star>,
    moon: Moon,
}

impl Night {
    fn new(star_count: usize, mut rng: StdRng) -> Self {
        Night {
            stars: (0..star_count)
                .map(|_| Night::random_star(&mut rng))
                .collect(),
            moon: Moon {
                spots: (0..SPOT_COUNT)
                    .map(|_| Night::random_spot(&mut rng))
                    .collect(),
            },
        }
    }

    fn random_star(rng: &mut StdRng) -> Star {
        const STAR_COLORS: [Color; 5] = [
            Palette::DARK_GRAY,
            Palette::GRAY,
            Palette::BEIGE,
            Palette::WHITE,
            Palette::RED,
        ];
        Star {
            position: (rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0)),
            color: STAR_COLORS[rng.gen_range(0, STAR_COLORS.len())],
            radius: rng.gen_range(0.4, 1.0),
        }
    }

    fn random_spot(rng: &mut StdRng) -> Spot {
        Spot {
            placement: rng.gen_range(0.0, 1.0),
            angle: rng.gen_range(0.0, 1.0),
            radius: rng.gen_range(0.2, 1.0),
            color: match rng.gen_range(0.0, 1.0) {
                random if random < 0.45 => Palette::BEIGE,
                random if random < 0.90 => Palette::DARK_BEIGE,
                _ => Palette::DARKER_BEIGE,
            },
        }
    }
}

impl Drawable for Night {
    fn draw(&self, canvas: &mut skia_safe::Canvas) {
        canvas.clear(Palette::BLACK);
        let width = canvas.width();
        let height = canvas.height();
        let star_scale = 4.0;
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

        self.moon.draw(canvas);
    }
}

impl Drawable for Moon {
    fn draw(&self, canvas: &mut skia_safe::Canvas) {
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_color(Palette::BEIGE);
        let canvas_small_side = canvas.width().min(canvas.height());
        let moon_radius = canvas_small_side * 0.6 / 2.0;

        let moon_center = (
            canvas_small_side * 0.1 + moon_radius,
            canvas.height() - canvas_small_side * 0.1 - moon_radius,
        );

        canvas.draw_circle(moon_center, moon_radius, &paint);

        for spot in self.spots.iter() {
            paint.set_color(spot.color);
            paint.set_alpha(200);

            let spot_radius = moon_radius * spot.radius * SPOT_SCALE;
            let r = (moon_radius - spot_radius) * spot.placement.sqrt();
            let theta = spot.angle * 2.0 * std::f32::consts::PI;

            let spot_center = (
                moon_center.0 + r * theta.cos(),
                moon_center.1 + r * theta.sin(),
            );
            canvas.draw_circle(spot_center, spot_radius, &paint);
        }
    }
}

pub fn draw(canvas: &mut Canvas) {
    let rng = StdRng::seed_from_u64(42);
    let star_count = ((canvas.width() * canvas.height() * 0.0002) as usize).max(10);

    Night::new(star_count, rng).draw(canvas);
}
