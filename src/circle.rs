#[derive(PartialEq, PartialOrd, Debug)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: isize,
}

impl Circle {
    pub fn new(width: isize, heigth: isize) -> Self {
        let radius = isize::min(width, heigth) / 2 - 1; 
        Self {
            center: Point {
                x: (width + 1) / 2,
                y: (heigth + 1) / 2,
            },
            radius,
        }
    }

    pub fn get_point(&self, angle: f64, change: fn(isize, f64) -> f64) -> Point {
        Point {
            x: self.center.x + change(self.radius, f64::sin(angle)) as isize,
            y: self.center.y - change(self.radius, f64::cos(angle)) as isize
        }
    }
}

#[cfg(test)]
mod circle_test {
    use super::*;

    #[test]
    fn test_new() {
        let circle = Circle::new(10, 6);

        assert_eq!(circle.radius, 6 / 2 - 1);
        assert_eq!(circle.center, Point { x: 5, y: 3 });
    }
}
