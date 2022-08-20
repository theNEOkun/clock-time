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
pub fn draw_line_float(term: &mut Term, start: &Point, end: &Point, what: &StyledContent<&str>) {
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
}

/// Method to draw a line to the terminal
///
/// ## Arguments
/// * start - The point to start the line from
/// * end - The point to end the line
///
/// Both use the Point-struct
///
/// * what - What to write on the lines. Uses StyledContent
pub fn draw_line_int(term: &mut Term, start: &Point, end: &Point, what: &StyledContent<&str>) {
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
}

