use crossterm::style::StyledContent;

use crate::circle::Point;

use super::Term;


/// Method to draw a line to the terminal
///
/// ## Arguments
/// * start - The point to start the line from
/// * end - The point to end the line
///
/// Both use the Point-struct
///
/// * what - What to write on the lines. Uses StyledContent
pub fn draw_line(term: &mut Term, start: &Point, end: &Point, what: &StyledContent<&str>) {
    let dx = (end.x - start.x) as f64;
    let dy = (end.y - start.y) as f64;

    let len = f64::sqrt(dx * dx + dy * dy);

    let xinc = dx / len;
    let yinc = dy / len;

    let mut x = start.x as f64;
    let mut y = start.y as f64;

    for _ in 0..=len as i64 {
        term.put_pixel(x.round() as i16, y.round() as i16, what);
        x += xinc;
        y += yinc;
    }

    /*
    if isize::abs(end.y - start.y) < isize::abs(end.x - start.x) {
        if start.x > end.x {
            term.draw_line_low(end, start, what);
        } else {
            term.draw_line_low(start, end, what);
        }
    } else {
        if start.y > end.y {
            term.draw_line_high(end, start, what);
        } else {
            term.draw_line_high(start, end, what);
        }
    }
    */
}

/// Method to draw a line in the x-direction
fn draw_line_high(term: &mut Term, start: &Point, end: &Point, what: &StyledContent<&str>) {
    let dy = end.y - start.y;
    let mut dx = end.x - start.x;

    let mut xinc = 1;

    if dx < 0 {
        dx = -dx;
        xinc = -1
    }

    let mut d = (2 * dx) - dy;
    let mut x = start.x;

    for y in start.y..end.y {
        term.put_pixel(x as i16, y as i16, what);
        if d > 0 {
            x += xinc;
            d += 2 * (dx - dy);
        } else {
            d += 2 * dx;
        }
    }
}

/// Method to draw a line in the y-direction
fn draw_line_low(term: &mut Term, start: &Point, end: &Point, what: &StyledContent<&str>) {
    let dx = end.x - start.x;
    let mut dy = end.y - start.y;

    let mut yinc = 1;

    if dy < 0 {
        dy = -dy;
        yinc = -1
    }

    let mut d = (2 * dy) - dx;
    let mut y = start.y;

    for x in start.x..end.x {
        term.put_pixel(x as i16, y as i16, what);
        if d > 0 {
            y += yinc;
            d += 2 * (dy - dx);
        } else {
            d += 2 * dy;
        }
    }
}
