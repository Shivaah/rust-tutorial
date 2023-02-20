// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }

    fn magnitude(&self) -> f64 {
        f64::from(self.x * self.x + self.y * self.y).sqrt()
    }

    fn dist(&self, p: Point) -> f64 {
        let dx = p.x - self.x;
        let dy = p.y - self.y;

        f64::from(dx * dx + dy * dy).sqrt()
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub struct Polygon {
    points: Vec<Point>
}

impl Polygon {
    fn new() -> Self {
        Self { points: Vec::new() }
    }

    fn add_point(&mut self, p: Point) {
        self.points.push(p);
    }

    fn left_most_point(&self) -> Option<Point> {
        self.points.iter().min_by_key(| p | p.x).copied()
    }

    fn iter(&self) -> core::slice::Iter<Point> {
        self.points.iter()
    }

    fn perimeter(&self) -> f64 {
        if self.points.is_empty() {
            return 0.0
        }

        let mut sum: f64 = 0.0;
        let last_index = self.points.len();

        for idx in 0..=(self.points.len() - 1) {
            let p1 = self.points[idx % last_index];
            let p2 = self.points[(idx + 1) % last_index];    

            sum += p1.dist(p2);
        }

        sum
    }
}

pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    fn new(center: Point, radius: i32) -> Self {
        Self { center: center, radius: radius }
    }

    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * f64::from(self.radius)
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl Shape {
    fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle(circle) => circle.perimeter(),
            Shape::Polygon(polygon) => polygon.perimeter()
        }
    }
}


impl From<Circle> for Shape {
    fn from(value: Circle) -> Self {
        Shape::Circle(value)
    }
}

impl From<Polygon> for Shape {
    fn from(value: Polygon) -> Self {
        Shape::Polygon(value)
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

        let circle = Circle::new(Point::new(10, 20), 5);

        let shapes = vec![
            Shape::from(poly),
            Shape::from(circle),
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
fn main() { }