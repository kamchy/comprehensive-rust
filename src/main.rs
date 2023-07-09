use core::slice::Iter;
use std::f64::consts::PI;
use std::ops::Add;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn magnitude(self) -> f64 {
        (self.x.pow(2) as f64 + self.y.pow(2) as f64).sqrt()
    }

    fn dist(&self, other: Point) -> f64 {
        (((self.x - other.x) as f64).powi(2) + ((self.y - other.y) as f64).powi(2)).sqrt()
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    fn new() -> Self {
        Polygon { points: Vec::new() }
    }

    fn add_point(&mut self, p: Point) {
        self.points.push(p)
    }

    fn perimeter(&self) -> f64 {
        let mut sum: f64 = 0.0;
        let len = self.points.len();
        for i in 0..len - 1 {
            sum += self.points[i].dist(self.points[i + 1])
        }
        sum + self.points[len - 1].dist(self.points[0])
    }

    fn left_most_point(&self) -> Option<Point> {
        self.points.iter().min_by(|p, q| p.x.cmp(&q.x)).copied()
    }

    fn iter(&self) -> Iter<Point> {
        self.points.iter()
    }
}

pub struct Circle {
    pos: Point,
    radius: i32, // add fields
}

impl Circle {
    fn new(p: Point, r: i32) -> Self {
        Circle { radius: r, pos: p }
    }

    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius as f64
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl From<Circle> for Shape {
    fn from(c: Circle) -> Shape {
        Shape::Circle(c)
    }
}

impl From<Polygon> for Shape {
    fn from(c: Polygon) -> Shape {
        Shape::Polygon(c)
    }
}

impl Shape {
    fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle(c) => c.perimeter(),
            Shape::Polygon(p) => p.perimeter(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_perimeters() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let perimeters = shapes
            .iter()
            .map(Shape::perimeter)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(perimeters, vec![15.48, 31.42]);
    }
}

#[allow(dead_code)]
fn main() {}
