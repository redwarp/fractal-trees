use std::fmt::Display;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use skia_safe::{Canvas, Color, Paint, PaintStyle, Path};

use crate::geometry::Segment;
use crate::utils::Bounded;
use crate::utils::Drawable;

/// Background color, rendered behind the maze.
const BACKGROUND_COLOR: Color = Color::new(0xfffceccb);
/// The higher the number, the less complex the maze.
const MAZE_TO_PIXEL: f32 = 10.0;
/// Adjust border to frame the maze in a nice way.
const MAZE_BORDER: f32 = 40.0;
/// Adjust for wider or thinner walls.
const STROKE_WIDTH: f32 = 0.5;

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

/// Maze structure. Can you get out?
///
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
    fn new(width: u32, height: u32, mut rng: StdRng) -> Self {
        let maze = Maze {
            width,
            height,
            data: vec![Cell::blank(); ((width * 2 + 1) * (height * 2 + 1)) as usize],
        }
        .initialise_maze(&mut rng)
        .collapse_entry_and_exit(&mut rng);

        maze
    }

    /// Uses the Randomized depth-first search found on wikipedia (https://en.wikipedia.org/wiki/Maze_generation_algorithm)
    /// to fill in the maze.
    fn initialise_maze(mut self, rng: &mut StdRng) -> Self {
        let mut cell_positions: Vec<(u32, u32)> = Vec::new();
        // Initialize first cell position.
        cell_positions.push((rng.gen_range(0, self.width), rng.gen_range(0, self.height)));

        while !cell_positions.is_empty() {
            let current_cell = cell_positions.pop().unwrap();
            if let Some(neighboor_cell_position) = self.random_unvisted_neighboor(current_cell, rng)
            {
                cell_positions.push(current_cell);
                self.collapse_wall_between(current_cell, neighboor_cell_position);
                if let Some(cell) =
                    self.get_floor_cell_mut(neighboor_cell_position.0, neighboor_cell_position.1)
                {
                    cell.visited = true;
                    // As the maze is initialized with only wall, it's important to mark cells as floor.
                    // We could also do a pass initially to put floor everywhere, but it's not needed as this algo gives us
                    // the certainty that every cell will be visited anyway.
                    cell.cell_type = CellType::Floor;
                }
                cell_positions.push(neighboor_cell_position);
            }
        }

        self
    }

    fn get_floor_cell(&self, x: u32, y: u32) -> Option<&Cell> {
        if x >= self.width || y >= self.height {
            return None;
        }

        self.data
            .get(((y * 2 + 1) * (self.width * 2 + 1) + (x * 2 + 1)) as usize)
    }

    fn get_floor_cell_mut(&mut self, x: u32, y: u32) -> Option<&mut Cell> {
        if x >= self.width || y >= self.height {
            return None;
        }

        self.data
            .get_mut(((y * 2 + 1) * (self.width * 2 + 1) + (x * 2 + 1)) as usize)
    }

    fn get_any_cell(&self, true_x: usize, true_y: usize) -> Option<&Cell> {
        if true_x >= self.width as usize * 2 + 1 || true_y >= self.height as usize * 2 + 1 {
            return None;
        }

        self.data
            .get(true_y * (self.width as usize * 2 + 1) + true_x)
    }
    fn get_any_cell_mut(&mut self, true_x: usize, true_y: usize) -> Option<&mut Cell> {
        if true_x >= self.width as usize * 2 + 1 || true_y >= self.height as usize * 2 + 1 {
            return None;
        }

        self.data
            .get_mut(true_y * (self.width as usize * 2 + 1) + true_x)
    }

    fn collapse_wall_between(&mut self, position_a: (u32, u32), position_b: (u32, u32)) {
        let x = ((position_a.0 * 2 + position_b.0 * 2 + 2) / 2) as usize;
        let y = ((position_a.1 * 2 + position_b.1 * 2 + 2) / 2) as usize;
        let index = y * (self.width as usize * 2 + 1) + x;

        self.data[index].cell_type = CellType::Floor;
    }

    fn random_unvisted_neighboor(
        &self,
        position: (u32, u32),
        rng: &mut StdRng,
    ) -> Option<(u32, u32)> {
        let mut unvisited: Vec<(u32, u32)> = Vec::with_capacity(4);
        unvisited.push((position.0, position.1 + 1));
        if position.1 > 0 {
            unvisited.push((position.0, position.1 - 1));
        };
        unvisited.push((position.0 + 1, position.1));
        if position.0 > 0 {
            unvisited.push((position.0 - 1, position.1));
        }

        unvisited.retain(
            |position| match self.get_floor_cell(position.0, position.1) {
                Some(cell) => !cell.visited,
                None => false,
            },
        );

        match unvisited.len() {
            0 => None,
            _ => Some(unvisited[rng.gen_range(0, unvisited.len())]),
        }
    }

    fn collapse_entry_and_exit(mut self, rng: &mut StdRng) -> Self {
        let west_wall = rng.gen_range(0, self.height);
        let east_wall = rng.gen_range(0, self.height);

        if let Some(cell) = self.get_any_cell_mut(0, (west_wall * 2 + 1) as usize) {
            cell.cell_type = CellType::Floor;
        }
        if let Some(cell) =
            self.get_any_cell_mut((self.width * 2) as usize, (east_wall * 2 + 1) as usize)
        {
            cell.cell_type = CellType::Floor;
        }

        self
    }
}

pub fn draw(canvas: &mut Canvas) {
    // Using a set seed to have a reproducable maze.
    let rng = StdRng::seed_from_u64(42);

    canvas.clear(BACKGROUND_COLOR);
    let width = ((canvas.width() - MAZE_BORDER * 2.0) / MAZE_TO_PIXEL) as u32;
    let height = ((canvas.height() - MAZE_BORDER * 2.0) / MAZE_TO_PIXEL) as u32;

    let maze = Maze::new(width, height, rng);

    println!("{}", maze);
    maze.draw(canvas);
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut description = format!("Maze of dimension {}x{}", self.width, self.height);
        if self.width > 20 || self.height > 20 {
            return write!(f, "{}", description);
        };

        description.push('\n');

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

impl Drawable for Maze {
    fn draw(&self, canvas: &mut Canvas) {
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Stroke);

        canvas.save();
        canvas.translate((MAZE_BORDER, MAZE_BORDER));

        let scale_x = (canvas.width() - MAZE_BORDER * 2.0) / (self.width as f32 * 2.0 + 1.0);
        let scale_y = (canvas.height() - MAZE_BORDER * 2.0) / (self.height as f32 * 2.0 + 1.0);

        canvas.translate((0.5 * scale_x, 0.5 * scale_y));
        paint.set_stroke_width(STROKE_WIDTH);

        let half_stroke = STROKE_WIDTH * 0.45;

        canvas.scale((scale_x, scale_y));

        let mut segments: Vec<Segment> = Vec::new();
        let mut lines: Vec<((usize, usize), (usize, usize))> = Vec::new();
        let mut columns: Vec<((usize, usize), (usize, usize))> = Vec::new();

        let width = (self.width * 2 + 1) as usize;
        let height = (self.height * 2 + 1) as usize;

        for y in 0..height {
            let mut in_progress_segment: (Option<(usize, usize)>, Option<(usize, usize)>) =
                (None, None);
            for x in 0..width {
                let cell_type: CellType = self.get_any_cell(x, y).unwrap().cell_type;
                match cell_type {
                    CellType::Wall => {
                        // Start or continue segment
                        match in_progress_segment.0 {
                            None => in_progress_segment.0 = Some((x, y)),
                            Some(_) => in_progress_segment.1 = Some((x, y)),
                        }
                    }
                    CellType::Floor => {
                        // Finish segment
                        if let (Some(a), Some(b)) = in_progress_segment {
                            lines.push((a, b));
                        }
                        in_progress_segment = (None, None)
                    }
                }
                if let (Some(a), Some(b)) = in_progress_segment {
                    lines.push((a, b));
                }
            }
        }

        for x in 0..width {
            let mut in_progress_segment: (Option<(usize, usize)>, Option<(usize, usize)>) =
                (None, None);
            for y in 0..height {
                let cell_type: CellType = self.get_any_cell(x, y).unwrap().cell_type;
                match cell_type {
                    CellType::Wall => {
                        // Start or continue segment
                        match in_progress_segment.0 {
                            None => in_progress_segment.0 = Some((x, y)),
                            Some(_) => in_progress_segment.1 = Some((x, y)),
                        }
                    }
                    CellType::Floor => {
                        // Finish segment
                        if let (Some(a), Some(b)) = in_progress_segment {
                            columns.push((a, b));
                        }
                        in_progress_segment = (None, None)
                    }
                }
                if let (Some(a), Some(b)) = in_progress_segment {
                    columns.push((a, b));
                }
            }
        }

        segments.append(
            &mut lines
                .iter()
                .filter_map(|(a, b)| {
                    if a == b {
                        // No need to draw single walls.
                        None
                    } else {
                        Some(Segment::new(
                            a.0 as f32 - half_stroke,
                            a.1 as f32,
                            b.0 as f32 + half_stroke,
                            b.1 as f32,
                        ))
                    }
                })
                .collect(),
        );

        segments.append(
            &mut columns
                .iter()
                .filter_map(|(a, b)| {
                    if a == b {
                        // No need to draw single walls.
                        None
                    } else {
                        Some(Segment::new(
                            a.0 as f32,
                            a.1 as f32 - half_stroke,
                            b.0 as f32,
                            b.1 as f32 + half_stroke,
                        ))
                    }
                })
                .collect(),
        );

        let mut path = Path::new();

        for segment in segments {
            path.move_to(segment.a());
            path.line_to(segment.b());
        }
        canvas.draw_path(&path, &paint);

        canvas.restore();
    }
}
