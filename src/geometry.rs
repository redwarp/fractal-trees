use skia_safe::{Canvas, Paint, Point};
use vector2d::Vector2D;

pub struct Segment {
    xa: f32,
    ya: f32,
    xb: f32,
    yb: f32,
}

impl Segment {
    pub const fn new(xa: f32, ya: f32, xb: f32, yb: f32) -> Self {
        Self { xa, ya, xb, yb }
    }

    pub fn points(&self) -> (Point, Point) {
        (Point::new(self.xa, self.ya), Point::new(self.xb, self.yb))
    }

    pub fn center(&self) -> Point {
        Point::new(
            self.xa + (self.xb - self.xa) / 2.0,
            self.ya + (self.yb - self.ya) / 2.0,
        )
    }
}

impl From<Segment> for Vector2D<f32> {
    fn from(segment: Segment) -> Self {
        Vector2D::new(segment.xb - segment.xa, segment.yb - segment.ya)
    }
}

pub trait SegmentDrawing {
    fn draw_segment(&mut self, segment: Segment, paint: &Paint) -> ();
}

impl SegmentDrawing for Canvas {
    fn draw_segment(&mut self, segment: Segment, paint: &Paint) -> () {
        let (p1, p2) = segment.points();
        self.draw_line(p1, p2, paint);
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
}
