#[derive(PartialEq, PartialOrd, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: usize,
}

impl Circle {
    pub fn new(width: usize, heigth: usize) -> Self {
        let radius = usize::min(width, heigth) / 2 - 1; 
        Self {
            center: Point {
                x: (width + 1) / 2,
                y: (heigth + 1) / 2,
            },
            radius,
        }
    }

    pub fn get_point(&self, angle: f64, change: fn(usize, f64) -> f64) -> Point {
        Point {
            x: self.center.x + change(self.radius, angle.sin()).floor() as usize,
            y: self.center.y - change(self.radius, angle.cos()).floor() as usize
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
