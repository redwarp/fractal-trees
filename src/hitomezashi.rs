use crate::utils::{Bounded, Drawable, Palette};
use rand::prelude::StdRng;
use rand::{prelude::Rng, SeedableRng};
use skia_safe::{Canvas, Paint, PaintStyle, Path, Point, Rect};

/// The higher the number, the less complex the maze.
const TO_PIXEL_RATIO: f32 = 30.0;
/// Adjust border to frame the drawing in a nice way.
const BORDER: f32 = 40.0;
/// Adjust for wider or thinner walls.
const STROKE_WIDTH: f32 = 0.2;

#[derive(Debug)]
struct Hitomezashi {
    width: usize,
    height: usize,
    horizontal: Vec<bool>,
    vertical: Vec<bool>,
}

impl Hitomezashi {
    fn with_random(width: usize, height: usize, rng: &mut StdRng) -> Self {
        let mut horizontal = Vec::<bool>::with_capacity(width);
        let mut vertical = Vec::<bool>::with_capacity(height);
        for _ in 0..width {
            horizontal.push(rng.gen());
        }
        for _ in 0..height {
            vertical.push(rng.gen());
        }

        Hitomezashi {
            width,
            height,
            horizontal,
            vertical,
        }
    }
}

impl Drawable for Hitomezashi {
    fn draw(&self, canvas: &mut Canvas) {
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Stroke);

        canvas.save();
        canvas.translate((BORDER, BORDER));

        let scale_x = (canvas.width() - BORDER * 2.0) / (self.width as f32 + 1.0);
        let scale_y = (canvas.height() - BORDER * 2.0) / (self.height as f32 + 1.0);

        canvas.translate((0.5 * scale_x, 0.5 * scale_y));
        paint.set_stroke_width(STROKE_WIDTH);

        let half_stroke = STROKE_WIDTH * 0.45;

        canvas.scale((scale_x, scale_y));
        let mut positions: Vec<Segment> = vec![];

        let mut path = Path::new();
        for x in 0..self.horizontal.len() {
            for y in 0..self.vertical.len() {
                let has_horizontal_segment = (y != 0) & (!self.vertical[y] ^ (x % 2 == 0));
                let has_vertical_segment = (x != 0) & (!self.horizontal[x] ^ (y % 2 == 0));

                if has_horizontal_segment {
                    path.move_to((x as f32 - half_stroke, y as f32));
                    path.line_to((x as f32 + 1.0 + half_stroke, y as f32));
                    positions.push(Segment {
                        a: Position { x, y },
                        b: Position { x: x + 1, y },
                    })
                }
                if has_vertical_segment {
                    path.move_to((x as f32, y as f32 - half_stroke));
                    path.line_to((x as f32, y as f32 + 1.0 + half_stroke));
                    positions.push(Segment {
                        a: Position { x, y },
                        b: Position { x, y: y + 1 },
                    })
                }
            }
        }

        let paths = segments_to_paths(&positions);

        let colors = vec![
            Palette::BLACK,
            Palette::GRAY,
            Palette::DARK_GRAY,
            Palette::DARK_BEIGE,
            Palette::DARKER_BEIGE,
        ];

        for (index, path) in paths.iter().enumerate() {
            paint.set_color(colors[index % colors.len()]);

            canvas.draw_path(path, &paint);
        }

        // canvas.draw_path(&path, &paint);
        let outer_rect = Rect {
            left: 0.0,
            top: 0.0,
            right: self.horizontal.len() as f32,
            bottom: self.vertical.len() as f32,
        };
        paint.set_color(Palette::BLACK);
        canvas.draw_rect(outer_rect, &paint);

        canvas.restore();
    }
}

pub fn draw(canvas: &mut Canvas) {
    // Using a set seed to have a reproducable maze.
    let mut rng = StdRng::seed_from_u64(42);

    canvas.clear(Palette::BEIGE);
    let width = ((canvas.width() - BORDER * 2.0) / TO_PIXEL_RATIO) as usize;
    let height = ((canvas.height() - BORDER * 2.0) / TO_PIXEL_RATIO) as usize;

    let hitomezashi = Hitomezashi::with_random(width, height, &mut rng);
    // let hitomezashi = Hitomezashi::with_random(10, 10, &mut rng);

    hitomezashi.draw(canvas);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl From<Position> for Point {
    fn from(position: Position) -> Self {
        Point {
            x: position.x as f32,
            y: position.y as f32,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Segment {
    a: Position,
    b: Position,
}

#[derive(Debug, Clone)]
struct Line {
    positions: Vec<Position>,
}

impl Line {
    fn new(segment: Segment) -> Self {
        Line {
            positions: vec![segment.a, segment.b],
        }
    }

    fn append_segment(&mut self, segment: &Segment) -> bool {
        {
            let last_point = self.positions.last().unwrap();
            if &segment.a == last_point {
                self.positions.push(segment.b);
                return true;
            } else if &segment.b == last_point {
                self.positions.push(segment.a);
                return true;
            }
        }
        {
            let first_point = self.positions.first().unwrap();
            if &segment.a == first_point {
                self.positions.insert(0, segment.b);
                return true;
            } else if &segment.b == first_point {
                self.positions.insert(0, segment.a);
                return true;
            }
        }
        false
    }

    fn append_line(&mut self, line: &Line) -> bool {
        if self.positions[0] == *line.positions.last().unwrap() {
            for (index, position) in line
                .positions
                .iter()
                .take(line.positions.len() - 1)
                .enumerate()
            {
                self.positions.insert(index, *position);
            }
            return true;
        } else if *self.positions.last().unwrap() == line.positions[0] {
            self.positions.extend_from_slice(&line.positions[1..]);
            return true;
        } else if self.positions[0] == line.positions[0] {
            for position in line.positions.iter().skip(1) {
                self.positions.insert(0, *position);
            }
            return true;
        } else if self.positions.last() == line.positions.last() {
            for position in line.positions.iter().rev().skip(1) {
                self.positions.push(*position);
            }
            return true;
        }

        false
    }
}

fn segments_to_paths(segments: &Vec<Segment>) -> Vec<Path> {
    let mut all_lines: Vec<Line> = vec![];
    println!("We have {} segments", segments.len());
    for segment in segments {
        let mut appended = false;
        for existing_line in all_lines.iter_mut() {
            if existing_line.append_segment(segment) {
                appended = true;
                break;
            }
        }

        if !appended {
            all_lines.push(Line::new(*segment));
        }
    }
    let count = all_lines
        .iter()
        .fold(0, |acc, line| line.positions.len() - 1 + acc);
    println!("Attributed to lines: {}", count);

    let mut done = false;
    while !done {
        let (did_fold, lines) = join_lines(all_lines);
        all_lines = lines;
        done = !did_fold;
    }

    all_lines
        .iter()
        .map(|line| {
            let line = &line.positions;
            let is_closed = line.len() > 1 && line[0] == *line.last().unwrap();
            let mut path = Path::new();
            let origin = (line[0].x as f32, line[0].y as f32);
            path.move_to(origin);
            for position in line.iter().take(if is_closed {
                line.len() - 1
            } else {
                line.len()
            }) {
                path.line_to(*position);
            }
            if is_closed {
                path.close();
            }

            path
        })
        .collect()
}

fn join_lines(lines: Vec<Line>) -> (bool, Vec<Line>) {
    let mut did_fold = false;

    let folded = lines
        .iter()
        .fold(Vec::new(), |mut folded: Vec<Line>, line| {
            let mut appended = false;
            for folded_line in folded.iter_mut() {
                if folded_line.append_line(line) {
                    appended = true;
                    did_fold = true;
                    break;
                }
            }
            if !appended {
                folded.push(line.clone());
            }

            folded
        });

    (did_fold, folded)
}
