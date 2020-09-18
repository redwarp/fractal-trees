use skia_safe::{Canvas, Paint, Point};
use vector2d::Vector2D;

#[derive(Copy, Clone, Debug)]
pub struct Segment {
    pub xa: f32,
    pub ya: f32,
    pub xb: f32,
    pub yb: f32,
}

impl Segment {
    pub const fn new(xa: f32, ya: f32, xb: f32, yb: f32) -> Self {
        Self { xa, ya, xb, yb }
    }

    pub fn from_points(a: Point, b: Point) -> Self {
        Self {
            xa: a.x,
            ya: a.y,
            xb: b.x,
            yb: b.y,
        }
    }

    pub fn points(&self) -> (Point, Point) {
        (Point::new(self.xa, self.ya), Point::new(self.xb, self.yb))
    }

    pub fn center(&self) -> Point {
        self.point_at_position(0.5)
    }

    pub fn point_at_position(&self, position: f32) -> Point {
        Point::new(
            self.xa + (self.xb - self.xa) * position,
            self.ya + (self.yb - self.ya) * position,
        )
    }

    pub fn a(&self) -> Point {
        Point::new(self.xa, self.ya)
    }

    pub fn b(&self) -> Point {
        Point::new(self.xb, self.yb)
    }

    pub fn to_vector2d(&self) -> Vector2D<f32> {
        Vector2D::new(self.xb - self.xa, self.yb - self.ya)
    }

    pub fn normal(&self) -> Vector2D<f32> {
        let vector: Vector2D<f32> = self.to_vector2d();
        vector.normal()
    }

    pub fn length(&self) -> f32 {
        self.to_vector2d().length()
    }
}

impl From<Segment> for Vector2D<f32> {
    fn from(segment: Segment) -> Self {
        Vector2D::new(segment.xb - segment.xa, segment.yb - segment.ya)
    }
}

impl From<Segment> for Line {
    fn from(segment: Segment) -> Self {
        let m = (segment.yb - segment.ya) / (segment.xb - segment.xa);
        let p = segment.ya - m * segment.xa;
        Line::new(m, p)
    }
}

pub trait ExtendedDraw {
    fn draw_segment(&mut self, segment: Segment, paint: &Paint);
}

impl ExtendedDraw for Canvas {
    fn draw_segment(&mut self, segment: Segment, paint: &Paint) {
        let (p1, p2) = segment.points();
        self.draw_line(p1, p2, paint);
    }
}

pub trait VectorMove {
    fn move_along(self, vector: Vector2D<f32>, distance: f32) -> Self;
}

impl VectorMove for Point {
    fn move_along(self, vector: Vector2D<f32>, distance: f32) -> Self {
        let vector = vector.normalise();
        Point::new(self.x + vector.x * distance, self.y + vector.y * distance)
    }
}

pub struct Line {
    pub m: f32,
    pub p: f32,
}

impl Line {
    pub fn new(m: f32, p: f32) -> Self {
        Self { m, p }
    }

    #[allow(dead_code)]
    pub fn intersection(self, other: Line) -> Result<Point, &'static str> {
        if (self.m - other.m).abs() < f32::EPSILON {
            return Err("The two lines are parallel");
        };

        let x = (self.p - other.p) / (other.m - self.m);
        let y = self.m * x + self.p;
        Ok(Point::new(x, y))
    }
}

#[cfg(test)]
mod test {
    use crate::geometry::*;

    #[test]
    fn vector() {
        let segment = Segment::new(0.0, 0.0, 2.0, 0.0);
        let vector: Vector2D<f32> = segment.into();

        assert_eq!(2.0, vector.x);
        assert_eq!(0.0, vector.y);
    }

    #[test]
    fn center() {
        let segment = Segment::new(1.0, 1.0, 4.0, 2.0);
        let center = segment.center();

        assert_eq!(2.5, center.x);
        assert_eq!(1.5, center.y);
    }

    #[test]
    fn center_negative_segment() {
        let segment = Segment::new(4.0, 2.0, 1.0, 1.0);
        let center = segment.center();

        assert_eq!(2.5, center.x);
        assert_eq!(1.5, center.y);
    }

    #[test]
    fn point_at_position_quarter() {
        let segment = Segment::new(1.0, 1.0, 4.0, 2.0);
        let center = segment.point_at_position(0.25);

        assert_eq!(1.75, center.x);
        assert_eq!(1.25, center.y);
    }

    #[test]
    fn point_at_position_quarter_of_negative_segment() {
        let segment = Segment::new(4.0, 2.0, 1.0, 1.0);
        let center = segment.point_at_position(0.25);

        assert_eq!(3.25, center.x);
        assert_eq!(1.75, center.y);
    }

    #[test]
    fn normal() {
        let segment = Segment::new(1.0, 1.0, 4.0, 1.0);
        let normal = segment.normal();

        assert_eq!(0.0, normal.x);
        assert_eq!(3.0, normal.y);
    }

    #[test]
    fn move_along_point() {
        let point = Point::new(10.0, 7.0);
        let moved_point = point.move_along(Vector2D::new(2.0, 0.0), 5.0);

        assert_eq!(15.0, moved_point.x);
        assert_eq!(7.0, moved_point.y);
    }

    #[test]
    fn lines_intersection_has_intersection() {
        let line1 = Line::new(1.0, -4.0);
        let line2 = Line::new(-2.0, 5.0);
        let intersection = line1.intersection(line2).unwrap();

        assert_eq!(3.0, intersection.x);
        assert_eq!(-1.0, intersection.y);
    }

    #[test]
    fn segment_into_line() {
        let line: Line = Segment::new(0.0, 0.0, 2.0, 1.0).into();

        assert_eq!(0.5, line.m);
        assert_eq!(0.0, line.p);
    }
}
