#[derive(PartialEq, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub struct Circle {
    pub center: Point,
    pub radius: usize,
}

impl Circle {
    pub fn new(width: usize, heigth: usize) -> Self {
        let radius = usize::min(width, heigth) / 2;
        Self {
            center: Point {
                x: radius,
                y: radius,
            },
            radius,
        }
    }

    pub fn get_point(&self, angle: usize, change: fn(usize, usize) -> usize) -> Point {
        Point {
            x: self.center.x + (change(self.radius, angle) as f64).sin() as usize,
            y: self.center.y - (change(self.radius, angle) as f64).cos() as usize
        }
    }
}

#[cfg(test)]
mod circle_test {
    use super::*;

    #[test]
    fn test_new() {
        let circle = Circle::new(10, 5);

        assert_eq!(circle.radius, 5 / 2);
        assert_eq!(circle.center, Point { x: 2, y: 2 });
    }
}
