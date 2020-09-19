use skia_safe::{Canvas, Color, Matrix, Paint, Rect};

use crate::utils::{Bounded, Palette};

const ANG: f64 = 20.0;
const BASE_LENGTH: f32 = 10.0;
const SKY_COLOR: Color = Palette::BEIGE;
const TREE_AND_EARTH_COLOR: Color = Palette::BLACK;
const ROOT_COLOR: Color = Palette::RED;
const DEPTH: u32 = 10;
const IDEAL_WIDTH: f32 = 1920.0;
const IDEAL_HEIGHT: f32 = 1080.0;

pub fn draw(canvas: &mut Canvas) {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);

    let width = IDEAL_WIDTH;
    let height = IDEAL_HEIGHT;

    let tree_depth = DEPTH;
    let root_depth = (DEPTH * 3) / 4;

    // Fill with the sky color.
    canvas.clear(SKY_COLOR);

    // Calculate how big the tree and roots will be, so we can then draw them at the proper space.
    let mut tree_rect = Rect::new(0.0, 0.0, 0.0, 0.0);
    let mut root_rect = Rect::new(0.0, 0.0, 0.0, 0.0);

    let mut calc_tree = |x1: f32, y1: f32, x2: f32, y2: f32, _depth: u32, rect: &mut Rect| {
        bound_branch(x1, y1, x2, y2, rect);
    };
    parse_fractal_tree(
        0.0,
        0.0,
        0.0,
        tree_depth,
        BASE_LENGTH,
        &mut tree_rect,
        &mut calc_tree,
    );

    parse_fractal_tree(
        0.0,
        0.0,
        0.0,
        root_depth,
        BASE_LENGTH * 0.75,
        &mut root_rect,
        &mut calc_tree,
    );

    // Set the center of the tree, and earth level, so that the drawing will be perfectly centered.
    let tree_trunk_x = width / 2.0 - tree_rect.center_x();
    let earth_level = (height + tree_rect.height() - root_rect.height()) / 2.0;

    let scale = canvas.height() / height;
    let shift_x = -(width * scale - canvas.width()) / 2.0;

    let mut matrix = Matrix::new_identity();
    matrix.post_scale((scale, scale), None);
    matrix.post_translate((shift_x, 0.0));
    canvas.set_matrix(&matrix);

    // Draw the ground.
    canvas.draw_rect(
        Rect::new(
            shift_x / scale,
            earth_level,
            width + shift_x / scale,
            height / scale,
        ),
        &paint,
    );

    // Draw the upper tree.
    paint.set_color(TREE_AND_EARTH_COLOR);
    let mut draw = |x1: f32, y1: f32, x2: f32, y2: f32, depth: u32, paint: &mut Paint| {
        paint.set_stroke_width((depth as f32).powf(1.1));
        let first = (x1 as f32, y1 as f32);
        let second = (x2 as f32, y2 as f32);
        canvas.draw_line(first, second, &paint);
    };

    parse_fractal_tree(
        tree_trunk_x,
        earth_level,
        0.0,
        tree_depth,
        BASE_LENGTH,
        &mut paint,
        &mut draw,
    );

    // Draw the roots
    paint.set_color(ROOT_COLOR);

    parse_fractal_tree(
        tree_trunk_x,
        earth_level,
        180.0,
        root_depth,
        BASE_LENGTH * 0.75,
        &mut paint,
        &mut draw,
    );
}

fn parse_fractal_tree<Block, Param>(
    x1: f32,
    y1: f32,
    angle: f64,
    depth: u32,
    base_length: f32,
    param: &mut Param,
    block: &mut Block,
) where
    Block: FnMut(f32, f32, f32, f32, u32, &mut Param),
{
    let x2 = x1 + angle.to_radians().sin() as f32 * depth as f32 * base_length;
    let y2 = y1 - angle.to_radians().cos() as f32 * depth as f32 * base_length;

    block(x1, y1, x2, y2, depth, param);

    let alternate = if depth % 2 == 0 { 1.0 } else { -1.0 };

    if depth > 0 {
        parse_fractal_tree(
            x2,
            y2,
            angle - ANG,
            depth - 1,
            base_length * (1.0 + alternate * 0.1),
            param,
            block,
        );
        parse_fractal_tree(
            x2,
            y2,
            angle + ANG,
            depth - 1,
            base_length * (1.0 - alternate * 0.1),
            param,
            block,
        );
    }
}

fn bound_branch(x1: f32, y1: f32, x2: f32, y2: f32, rect: &mut Rect) {
    let xmin = x1.min(x2);
    let ymin = y1.min(y2);
    let xmax = x1.max(x2);
    let ymax = y1.max(y2);

    rect.left = if xmin < rect.left { xmin } else { rect.left };
    rect.right = if xmax > rect.right { xmax } else { rect.right };
    rect.top = if ymin < rect.top { ymin } else { rect.top };
    rect.bottom = if ymax > rect.bottom {
        ymax
    } else {
        rect.bottom
    };
}
