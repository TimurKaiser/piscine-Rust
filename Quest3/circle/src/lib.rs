#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[derive(Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            center: Point { x, y },
            radius,
        }
    }

    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }

    pub fn area(&self) -> f64 {
        let pi = std::f64::consts::PI;
        pi * self.radius.powf(2.0)
    }

    pub fn intersect(&self, other: &Circle) -> bool {
        let dx = self.center.x - other.center.x;
        let dy = self.center.y - other.center.y;
        let distance_between_centers = (dx * dx + dy * dy).sqrt();
        distance_between_centers < (self.radius + other.radius)
    }
}
