use std::fmt::Display;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use skia_safe::{Canvas, Color, Paint, PaintStyle, Point};

use crate::geometry::{ExtendedDraw, Segment};
use crate::utils::Bounded;
use crate::utils::Drawable;

const SKY_COLOR: Color = Color::new(0xfffceccb);
const MAZE_TO_PIXEL: f32 = 10.0;
const MAZE_BORDER: f32 = 20.0;

#[derive(Copy, Clone)]
struct Cell {
    visited: bool,
    cell_type: CellType,
}

impl Cell {
    fn blank() -> Cell {
        Cell {
            visited: false,
            cell_type: CellType::Wall,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum CellType {
    Wall,
    Floor,
}

/// A maze or labyrinth of size 3 * 3 might generate the following structure
///
/// ```text
/// # # # # # # #
/// #       #   #
/// #   # # #   #
/// #       #   #
/// # # #   #   #
/// #           #
/// # # # # # # #
/// ```
struct Maze {
    width: u32,
    height: u32,
    data: Vec<Cell>,
}

impl Maze {
    fn new(width: u32, height: u32, rng: StdRng) -> Self {
        let maze = Maze {
            width: width,
            height: height,
            data: vec![Cell::blank(); ((width * 2 + 1) * (height * 2 + 1)) as usize],
        }
        .initialise_maze(rng);

        maze
    }

    fn initialise_maze(mut self, mut rng: StdRng) -> Self {
        let mut cell_positions: Vec<(u32, u32)> = Vec::new();
        // Initialize first cell position.
        cell_positions.push((rng.gen_range(0, self.width), rng.gen_range(0, self.height)));

        while !cell_positions.is_empty() {
            let current_cell = cell_positions.pop().unwrap();
            match self.random_unvisted_neighboor(current_cell, &mut rng) {
                Some(other_cell) => {
                    cell_positions.push(current_cell);
                    self.collapse_wall_between(current_cell, other_cell);
                    if let Some(cell) = self.get_cell_mut(other_cell.0, other_cell.1) {
                        cell.visited = true;
                        cell.cell_type = CellType::Floor;
                    }
                    cell_positions.push(other_cell);
                }
                None => {}
            }
        }

        self
    }

    fn get_cell(&self, x: u32, y: u32) -> Option<&Cell> {
        if x >= self.width || y >= self.height {
            return None;
        }

        self.data
            .get(((y * 2 + 1) * (self.width * 2 + 1) + (x * 2 + 1)) as usize)
    }

    fn get_cell_mut(&mut self, x: u32, y: u32) -> Option<&mut Cell> {
        if x >= self.width || y >= self.height {
            return None;
        }

        self.data
            .get_mut(((y * 2 + 1) * (self.width * 2 + 1) + (x * 2 + 1)) as usize)
    }

    fn get_any_cell(&self, true_x: usize, true_y: usize) -> &Cell {
        &self.data[true_y * (self.width as usize * 2 + 1) + true_x]
    }

    fn collapse_wall_between(&mut self, cell_a: (u32, u32), cell_b: (u32, u32)) {
        let x = ((cell_a.0 * 2 + cell_b.0 * 2 + 2) / 2) as usize;
        let y = ((cell_a.1 * 2 + cell_b.1 * 2 + 2) / 2) as usize;
        let index = y * (self.width as usize * 2 + 1) + x;

        self.data[index].cell_type = CellType::Floor;
    }

    fn random_unvisted_neighboor(&self, cell: (u32, u32), rng: &mut StdRng) -> Option<(u32, u32)> {
        let mut unvisited: Vec<(u32, u32)> = Vec::with_capacity(4);
        unvisited.push((cell.0, cell.1 + 1));
        if cell.1 > 0 {
            unvisited.push((cell.0, cell.1 - 1));
        };
        unvisited.push((cell.0 + 1, cell.1));
        if cell.0 > 0 {
            unvisited.push((cell.0 - 1, cell.1));
        }

        unvisited.retain(
            |cell_position| match self.get_cell(cell_position.0, cell_position.1) {
                Some(cell) => !cell.visited,
                None => false,
            },
        );

        match unvisited.len() {
            0 => None,
            _ => Some(unvisited[rng.gen_range(0, unvisited.len())]),
        }
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut description = String::new();
        for (pos, cell) in self.data.iter().enumerate() {
            description.push(match cell.cell_type {
                CellType::Floor => ' ',
                CellType::Wall => '#',
            });
            if (pos + 1) % (self.width * 2 + 1) as usize == 0 && pos > 0 {
                description.push('\n');
            } else {
                description.push(' ')
            }
        }

        write!(f, "{}", description)
    }
}

pub fn draw(canvas: &mut Canvas) {
    let rng = StdRng::seed_from_u64(42);
    // let rng = StdRng::from_entropy();

    canvas.clear(SKY_COLOR);
    let width = ((canvas.width() - MAZE_BORDER * 2.0) / MAZE_TO_PIXEL) as u32;
    let height = ((canvas.height() - MAZE_BORDER * 2.0) / MAZE_TO_PIXEL) as u32;

    let maze = Maze::new(width, height, rng);

    // println!("{}", maze);
    maze.draw(canvas);
}

impl Drawable for Maze {
    fn draw(&self, canvas: &mut Canvas) {
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Stroke);

        canvas.save();
        canvas.translate((MAZE_BORDER, MAZE_BORDER));
        let scale_x = (canvas.width() - MAZE_BORDER * 2.0)
            / ((canvas.width() - MAZE_BORDER * 2.0) / MAZE_TO_PIXEL * 2.0 + 1.0);
        let scale_y = (canvas.height() - MAZE_BORDER * 2.0)
            / ((canvas.height() - MAZE_BORDER * 2.0) / MAZE_TO_PIXEL * 2.0 + 1.0);

        paint.set_stroke_width(0.5);

        canvas.scale((scale_x, scale_y));

        let width = (self.width * 2 + 1) as usize;
        let height = (self.height * 2 + 1) as usize;

        for y in 0..height {
            let mut current_x = 0;
            let mut segment_started = false;
            let mut segment_finished = false;

            let mut origin = Point::new(current_x as f32, y as f32);
            let mut end = origin.clone();
            while current_x < width - 1 {
                for x in current_x..width {
                    current_x = x;
                    let cell_type: CellType = self.get_any_cell(x, y).cell_type;
                    match (cell_type, segment_started) {
                        (CellType::Wall, false) => {
                            origin = Point::new(x as f32, y as f32);
                            segment_started = true;
                        }
                        (CellType::Floor, true) => {
                            end = Point::new((x - 1) as f32, y as f32);
                            segment_finished = true;
                            break;
                        }
                        _ => {}
                    };
                }
                if current_x == width - 1 && segment_started || !segment_finished {
                    end = Point::new((width - 1) as f32, y as f32);
                    segment_finished = true;
                }

                if origin != end {
                    let segment = Segment::from_points(origin, end);
                    canvas.draw_segment(segment, &paint);
                    segment_started = false;
                    segment_finished = false;
                } else {
                    current_x = current_x + 1;
                    segment_started = false;
                    segment_finished = false;
                }
            }
        }

        for x in 0..width {
            let mut current_y = 0;
            let mut segment_started = false;
            let mut segment_finished = false;

            let mut origin = Point::new(x as f32, current_y as f32);
            let mut end = origin.clone();
            while current_y < height - 1 {
                for y in current_y..height {
                    current_y = y;
                    let cell_type: CellType = self.get_any_cell(x, y).cell_type;
                    match (cell_type, segment_started) {
                        (CellType::Wall, false) => {
                            origin = Point::new(x as f32, y as f32);
                            segment_started = true;
                        }
                        (CellType::Floor, true) => {
                            end = Point::new(x as f32, (y - 1) as f32);
                            segment_finished = true;
                            break;
                        }
                        _ => {}
                    };
                }
                if current_y == height - 1 && segment_started || !segment_finished {
                    end = Point::new(x as f32, (height - 1) as f32);
                    segment_finished = true;
                }

                if origin != end {
                    let segment = Segment::from_points(origin, end);
                    canvas.draw_segment(segment, &paint);
                    segment_started = false;
                    segment_finished = false;
                } else {
                    current_y = current_y + 1;
                    segment_started = false;
                    segment_finished = false;
                }
            }
        }

        canvas.restore();
    }
}
