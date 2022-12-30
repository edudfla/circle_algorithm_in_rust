type NumberType = i64;

fn main() {
    //let radius: NumberType = (i64::MAX as f64).sqrt() as i64;
    let radius: NumberType = 1000000;
    let radius2: NumberType = radius * radius;
    // Starts pointing horizontally to the right.
    let mut x: NumberType = radius;
    let mut x2: NumberType = x * x;
    let mut y: NumberType = 0;
    let mut y2: NumberType = 0;
    let mut rounded_area: NumberType = y;
    
    //println!("{} {} {} {}", x, y, x, y);
    while y < x {
        // At every iteration y always increase one single unit.
        y2 += 2 * y + 1;
        y += 1;
        // x changes depending on the comparison of the old and new (x2 + y2) candidates.
        let new_x2: NumberType =  x2 - 2 * x + 1;
        if ((new_x2 + y2) - radius2).abs() < ((x2 + y2) - radius2).abs() {

            let angle = (((x * ( y - 1)) / 2 + rounded_area) * 2) as f64 / (radius * radius) as f64;
            let x_for_angle = (radius as f64 * f64::cos(angle)).round() as i64;
            let y_for_angle = (radius as f64 * f64::sin(angle)).round() as i64;
            if x != x_for_angle || (y -1 ) != y_for_angle {
                println!("{} {} {} {}", x, y - 1, x - x_for_angle, y - y_for_angle);
            }

            rounded_area += y - 1;
            x2 = new_x2;
            x -= 1;
        }
    }
    let angle = (((x * y) / 2 + rounded_area) * 2) as f64 / (radius * radius) as f64;
    let x_for_angle = (radius as f64 * f64::cos(angle)).round() as i64;
    let y_for_angle = (radius as f64 * f64::sin(angle)).round() as i64;
    if x != x_for_angle || y != y_for_angle {
        println!("{} {} {} {}", x, y - 1, x - x_for_angle, y - y_for_angle);
    }
}
