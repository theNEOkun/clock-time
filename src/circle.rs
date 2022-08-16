#[derive(PartialEq, Debug)]
pub struct Point {
    x: f64,
    y: f64,
}

pub struct Circle {
    center: Point,
    radius: f64,
}

impl Circle {
    pub fn new(width: f64, heigth: f64) -> Self {
        let radius = (f64::min(width, heigth) / 2.0).floor();
        Self {
            center: Point {
                x: radius,
                y: radius,
            },
            radius,
        }
    }

    pub fn get_point(&self, angle: f64, change: fn(f64, f64) -> f64) -> Point {
        Point {
            x: self.center.x + (change(self.radius, angle)).sin(),
            y: self.center.y - (change(self.radius, angle)).cos()
        }
    }
}

#[cfg(test)]
mod circle_test {
    use super::*;

    #[test]
    fn test_new() {
        let circle = Circle::new(10.0, 5.0);

        assert_eq!(circle.radius, f64::floor(5.0 / 2.0));
        assert_eq!(circle.center, Point { x: 2.0, y: 2.0 });
    }
}
