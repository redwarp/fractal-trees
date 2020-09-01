use std::fs::create_dir_all;
use std::fs::File;
use std::io::Write;

use skia_safe::{Canvas, Color, EncodedImageFormat, Paint, Surface};

const ANG: f64 = 20.0;
const BASE_LENGTH: f64 = 10.0;

#[derive(Debug)]
struct Rect {
    left: f64,
    top: f64,
    right: f64,
    bottom: f64,
}

impl Rect {
    fn new() -> Self {
        Rect {
            left: 0.0,
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
        }
    }

    fn center_x(&self) -> f64 {
        (self.right + self.left) / 2.0
    }
}

fn main() {
    let width = 1920;
    let height = 1080;
    let mut paint = Paint::default();
    paint.set_anti_alias(true);

    let mut surface =
        Surface::new_raster_n32_premul((width, height)).expect("No SKIA surface available.");

    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    let mut tree_rect = Rect::new();

    println!("Before {:?}", tree_rect);

    let mut calc = |x1: f64, y1: f64, x2: f64, y2: f64, _depth: u32| {
        let xmin = min(x1, x2);
        let ymin = min(y1, y2);
        let xmax = max(x1, x2);
        let ymax = max(y1, y2);

        tree_rect.left = if xmin < tree_rect.left {
            xmin
        } else {
            tree_rect.left
        };
        tree_rect.right = if xmax > tree_rect.right {
            xmax
        } else {
            tree_rect.right
        };
        tree_rect.top = if ymin < tree_rect.top {
            ymin
        } else {
            tree_rect.top
        };
        tree_rect.bottom = if ymax > tree_rect.bottom {
            ymax
        } else {
            tree_rect.bottom
        };
    };
    let mut draw = |x1: f64, y1: f64, x2: f64, y2: f64, depth: u32| {
        paint.set_stroke_width(depth as f32);
        let first = (x1 as f32, y1 as f32);
        let second = (x2 as f32, y2 as f32);
        canvas.draw_line(first, second, &paint);
    };

    parse_fractal_tree(0.0, 0.0, 0.0, 8, BASE_LENGTH, &mut calc);

    let tree_trunk_x = width as f64 / 2.0 - tree_rect.center_x();

    parse_fractal_tree(tree_trunk_x, height as f64, 0.0, 8, BASE_LENGTH, &mut draw);

    println!("After {:?}", tree_rect);

    create_dir_all("rendering").unwrap();
    let file_name = "rendering/tree.png";
    let mut file = File::create(file_name).unwrap();
    let image = surface.image_snapshot();
    match image.encode_to_data(EncodedImageFormat::PNG) {
        Some(data) => {
            file.write_all(data.as_bytes()).unwrap();
        }
        None => {
            eprintln!("ERROR: failed to encode image as PNG.");
        }
    }
}

fn parse_fractal_tree<Block>(
    x1: f64,
    y1: f64,
    angle: f64,
    depth: u32,
    base_length: f64,
    block: &mut Block,
) where
    Block: FnMut(f64, f64, f64, f64, u32),
{
    let x2 = x1 + angle.to_radians().sin() * depth as f64 * base_length;
    let y2 = y1 - angle.to_radians().cos() * depth as f64 * base_length;

    block(x1, y1, x2, y2, depth);

    let alternate = if depth % 2 == 0 { 1.0 } else { -1.0 };

    if depth > 0 {
        parse_fractal_tree(
            x2,
            y2,
            angle - ANG,
            depth - 1,
            base_length * (1.0 + alternate * 0.1),
            block,
        );
        parse_fractal_tree(
            x2,
            y2,
            angle + ANG,
            depth - 1,
            base_length * (1.0 - alternate * 0.1),
            block,
        );
    }
}

#[allow(dead_code)]
fn draw_fractal_tree(
    x1: f64,
    y1: f64,
    angle: f64,
    depth: u32,
    base_length: f64,
    canvas: &mut Canvas,
    paint: &mut Paint,
) {
    let x2 = x1 + angle.to_radians().sin() * depth as f64 * base_length;
    let y2 = y1 - angle.to_radians().cos() * depth as f64 * base_length;

    paint.set_stroke_width(depth as f32);
    let first = (x1 as f32, y1 as f32);
    let second = (x2 as f32, y2 as f32);
    canvas.draw_line(first, second, &paint);

    let alternate = if depth % 2 == 0 { 1.0 } else { -1.0 };

    if depth > 0 {
        draw_fractal_tree(
            x2,
            y2,
            angle - ANG,
            depth - 1,
            base_length * (1.0 + alternate * 0.1),
            canvas,
            paint,
        );
        draw_fractal_tree(
            x2,
            y2,
            angle + ANG,
            depth - 1,
            base_length * (1.0 - alternate * 0.1),
            canvas,
            paint,
        );
    }
}

fn min(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

fn max(a: f64, b: f64) -> f64 {
    if a >= b {
        a
    } else {
        b
    }
}
