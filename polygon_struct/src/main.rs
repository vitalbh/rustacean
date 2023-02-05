
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point {
    x: i32,
    y: i32
}


impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point{x ,y}
    }
    pub fn dist(self, p: Point) -> f64 {
        (self - p).magnitude()
    }
    pub fn magnitude(self) -> f64 {
        f64::from(self.x.pow(2)+ self.y.pow(2)).sqrt()
    }
}
impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub struct Polygon {
    points: Vec<Point>
}

impl Polygon {
    pub fn new() -> Polygon {
        Polygon{
            points: Vec::new()
        }
    }
    pub fn add_point(&mut self, p: Point) {
        self.points.push(p)
    }

    pub fn left_most_point(&self) -> Option<Point> {
        self.points.iter().min_by_key(|p| p.x).copied()
    }
    pub fn length(&self) -> f64 {
        let mut result = 0.0;
        if self.points.is_empty() {
            return result
        }
        for i in 1..self.points.len() {
            result += self.points[i-1].dist(self.points[i]);
        }
        result += self.points[self.points.len()-1].dist(self.points[0]);
        result
    }


    pub fn iter(&self) -> impl Iterator<Item = &Point> {
        self.points.iter()
    }

}

pub struct Circle {
    point: Point,
    radius: i32
}

impl Circle {
    pub fn new(point: Point, radius: i32) -> Circle {
        Circle {
            point, radius
        }
    }
    pub fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * f64::from(self.radius)
    }

    pub fn dist(&self, o: &Self) -> f64 {
        self.point.dist(o.point)
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl From<Polygon> for Shape {
    fn from(poly: Polygon) -> Self {
        Shape::Polygon(poly)
    }
}

impl From<Circle> for Shape {
    fn from(circle: Circle) -> Self {
        Shape::Circle(circle)
    }
}
impl Shape {
    pub fn perimeter(&self) -> f64 {
        match self {
            Shape::Polygon(p) => p.length(),
            Shape::Circle(c) => c.circumference()
        }
    }
}

fn round_two_digits(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;



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


fn main() {
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

    println!("perimeters: {:?}", perimeters);

}