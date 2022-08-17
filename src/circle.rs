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
        let smallest = isize::min(width, heigth);
        let radius = (smallest / 2) - 1; 
        Self {
            center: Point {
                x: (smallest + 1) / 2,
                y: (smallest + 1) / 2,
            },
            radius,
        }
    }

    /**
     * Method to get a point inside the circle
     *
     * ## Arguments
     * * angle - the angle that the point is on
     * * change - is a function that changes both the angle given in this method, 
     * as well as the radius given through the struct.
     *
     * ## Returns
     *
     * The Point returned is placed along the radius with the placement gotten through the angle 
     */
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
