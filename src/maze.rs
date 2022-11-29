use std::fmt::Display;

use rand::{rngs::StdRng, Rng, SeedableRng};
use skia_safe::{Canvas, Paint, PaintStyle, Path};

use crate::geometry::Segment;
use crate::utils::{Bounded, Drawable, Palette};

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
struct Position(usize, usize);

struct Wall(Position, Position);

impl From<(usize, usize)> for Position {
    fn from(point: (usize, usize)) -> Self {
        Position(point.0, point.1)
    }
}

impl Wall {
    fn as_segment(&self, half_stroke: f32) -> Option<Segment> {
        let Wall(a, b) = self;

        if a.0 != b.0 {
            // Horizontal wall
            Some(Segment::new(
                a.0 as f32 - half_stroke,
                a.1 as f32,
                b.0 as f32 + half_stroke,
                b.1 as f32,
            ))
        } else if a.1 != b.1 {
            // Vertical wall
            Some(Segment::new(
                a.0 as f32,
                a.1 as f32 - half_stroke,
                b.0 as f32,
                b.1 as f32 + half_stroke,
            ))
        } else {
            None
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
    width: usize,
    height: usize,
    data: Vec<Cell>,
    path: Vec<Position>,
}

impl Maze {
    fn new(width: usize, height: usize, mut rng: StdRng) -> Self {
        Maze {
            width,
            height,
            data: vec![Cell::blank(); ((width * 2 + 1) * (height * 2 + 1)) as usize],
            path: Vec::new(),
        }
        .initialise_maze(&mut rng)
        .collapse_entry_and_exit(&mut rng)
        .solve()
    }

    /// Uses the Randomized depth-first search found on wikipedia (https://en.wikipedia.org/wiki/Maze_generation_algorithm)
    /// to fill in the maze.
    fn initialise_maze(mut self, rng: &mut StdRng) -> Self {
        let mut cell_positions: Vec<Position> = Vec::new();
        // Initialize first cell position.
        cell_positions.push(Position(
            rng.gen_range(0, self.width),
            rng.gen_range(0, self.height),
        ));

        while !cell_positions.is_empty() {
            let current_cell = cell_positions.pop().unwrap();
            if let Some(neighboor_cell_position) = self.random_unvisted_neighboor(current_cell, rng)
            {
                cell_positions.push(current_cell);
                self.collapse_wall_between(current_cell, neighboor_cell_position);
                if let Some(cell) = self.get_floor_cell_mut(neighboor_cell_position) {
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

    fn get_floor_cell(&self, position: Position) -> Option<&Cell> {
        let Position(x, y) = position;
        if x >= self.width || y >= self.height {
            return None;
        }

        self.data
            .get(((y * 2 + 1) * (self.width * 2 + 1) + (x * 2 + 1)) as usize)
    }

    fn get_floor_cell_mut(&mut self, position: Position) -> Option<&mut Cell> {
        let Position(x, y) = position;
        if x >= self.width || y >= self.height {
            return None;
        }

        self.data
            .get_mut(((y * 2 + 1) * (self.width * 2 + 1) + (x * 2 + 1)) as usize)
    }

    fn get_any_cell(&self, true_x: usize, true_y: usize) -> Option<&Cell> {
        if true_x > self.width as usize * 2 || true_y > self.height as usize * 2 {
            return None;
        }

        self.data
            .get(true_y * (self.width as usize * 2 + 1) + true_x)
    }
    fn get_any_cell_mut(&mut self, true_x: usize, true_y: usize) -> Option<&mut Cell> {
        if true_x > self.width as usize * 2 || true_y > self.height as usize * 2 {
            return None;
        }

        self.data
            .get_mut(true_y * (self.width as usize * 2 + 1) + true_x)
    }

    fn collapse_wall_between(&mut self, position_a: Position, position_b: Position) {
        let x = ((position_a.0 * 2 + position_b.0 * 2 + 2) / 2) as usize;
        let y = ((position_a.1 * 2 + position_b.1 * 2 + 2) / 2) as usize;
        let index = y * (self.width as usize * 2 + 1) + x;

        self.data[index].cell_type = CellType::Floor;
    }

    fn random_unvisted_neighboor(&self, position: Position, rng: &mut StdRng) -> Option<Position> {
        let mut unvisited: Vec<Position> = Vec::with_capacity(4);
        unvisited.push(Position(position.0, position.1 + 1));
        if position.1 > 0 {
            unvisited.push(Position(position.0, position.1 - 1));
        };
        unvisited.push(Position(position.0 + 1, position.1));
        if position.0 > 0 {
            unvisited.push(Position(position.0 - 1, position.1));
        }

        unvisited.retain(|position| match self.get_floor_cell(*position) {
            Some(cell) => !cell.visited,
            None => false,
        });

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

    fn any_unvisted_neighboor(&self, position: Position) -> Option<Position> {
        let mut unvisited: Vec<Position> = Vec::with_capacity(4);
        unvisited.push(Position(position.0, position.1 + 1));
        if position.1 > 0 {
            unvisited.push(Position(position.0, position.1 - 1));
        };
        unvisited.push(Position(position.0 + 1, position.1));
        if position.0 > 0 {
            unvisited.push(Position(position.0 - 1, position.1));
        }

        unvisited.retain(|&Position(x, y)| match self.get_any_cell(x, y) {
            Some(cell) => !cell.visited && cell.cell_type == CellType::Floor,
            None => false,
        });

        match unvisited.len() {
            0 => None,
            _ => Some(*unvisited.first().unwrap()),
        }
    }

    fn unvisit_all(&mut self) {
        for cell in self.data.iter_mut() {
            cell.visited = false;
        }
    }

    fn solve(mut self) -> Self {
        self.path.clear();
        let height = self.height * 2 + 1;
        let width = self.width * 2 + 1;
        self.unvisit_all();

        let mut start: Option<Position> = None;
        for y in 0..height {
            if let Some(cell) = self.get_any_cell(0, y) {
                if cell.cell_type == CellType::Floor {
                    start = Some(Position(0, y));
                    break;
                }
            }
        }
        if start.is_none() {
            return self;
        }

        let mut current = start.unwrap();
        self.path.push(current);
        self.get_any_cell_mut(current.0, current.1).unwrap().visited = true;

        while current.0 < width - 1 {
            current = self.path.pop().unwrap();
            if let Some(neighboor) = self.any_unvisted_neighboor(current) {
                self.path.push(current);
                if let Some(cell) = self.get_any_cell_mut(neighboor.0, neighboor.1) {
                    cell.visited = true;
                    self.path.push(neighboor);
                }
            }
        }
        self.path.push(current);

        self
    }
}

pub fn draw(canvas: &mut Canvas) {
    // Using a set seed to have a reproducable maze.
    let rng = StdRng::seed_from_u64(42);

    canvas.clear(Palette::BEIGE);
    let width = ((canvas.width() - MAZE_BORDER * 2.0) / MAZE_TO_PIXEL) as usize;
    let height = ((canvas.height() - MAZE_BORDER * 2.0) / MAZE_TO_PIXEL) as usize;

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

        let mut walls: Vec<Wall> = Vec::new();

        let width = (self.width * 2 + 1) as usize;
        let height = (self.height * 2 + 1) as usize;

        for y in 0..height {
            let mut in_progress_segment: (Option<Position>, Option<Position>) = (None, None);
            for x in 0..width {
                let cell_type: CellType = self.get_any_cell(x, y).unwrap().cell_type;
                match cell_type {
                    CellType::Wall => {
                        // Start or continue segment
                        match in_progress_segment.0 {
                            None => in_progress_segment.0 = Some(Position(x, y)),
                            Some(_) => in_progress_segment.1 = Some(Position(x, y)),
                        }
                    }
                    CellType::Floor => {
                        // Finish segment
                        if let (Some(a), Some(b)) = in_progress_segment {
                            walls.push(Wall(a, b));
                        }
                        in_progress_segment = (None, None)
                    }
                }
                if let (Some(a), Some(b)) = in_progress_segment {
                    walls.push(Wall(a, b));
                }
            }
        }

        for x in 0..width {
            let mut in_progress_segment: (Option<Position>, Option<Position>) = (None, None);
            for y in 0..height {
                let cell_type: CellType = self.get_any_cell(x, y).unwrap().cell_type;
                match cell_type {
                    CellType::Wall => {
                        // Start or continue segment
                        match in_progress_segment.0 {
                            None => in_progress_segment.0 = Some(Position(x, y)),
                            Some(_) => in_progress_segment.1 = Some(Position(x, y)),
                        }
                    }
                    CellType::Floor => {
                        // Finish segment
                        if let (Some(a), Some(b)) = in_progress_segment {
                            walls.push(Wall(a, b));
                        }
                        in_progress_segment = (None, None)
                    }
                }
                if let (Some(a), Some(b)) = in_progress_segment {
                    walls.push(Wall(a, b));
                }
            }
        }

        let mut path = Path::new();

        for segment in walls.iter().filter_map(|wall| wall.as_segment(half_stroke)) {
            path.move_to(segment.a());
            path.line_to(segment.b());
        }
        paint.set_color(Palette::BLACK);
        canvas.draw_path(&path, &paint);

        // Draw solution.
        if let (Some(start), Some(end)) = (self.path.first(), self.path.last()) {
            let mut path = Path::new();
            path.move_to((start.0 as f32 - 2.0, start.1 as f32));

            for position in &self.path[..] {
                path.line_to((position.0 as f32, position.1 as f32));
            }
            path.line_to((end.0 as f32 + 2.0, end.1 as f32));

            paint.set_color(Palette::RED);
            canvas.draw_path(&path, &paint);
        }

        canvas.restore();
    }
}
