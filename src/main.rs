type NumberType = i32;

fn main() {
    let radius: NumberType = 1000;
    let radius2: NumberType = radius * radius;
    // Starts pointing horizontally to the right.
    let mut x: NumberType = radius;
    let mut x2: NumberType = x * x;
    let mut y: NumberType = 0;
    let mut y2: NumberType = 0;
    
    println!("{} {}", x, y);
    while y < x {
        // At every iteration y always increase one single unit.
        y2 += 2 * y + 1;
        y += 1;
        // x changes depending on the comparison of the old and new (x2 + y2) candidates.
        let new_x2: NumberType =  x2 - 2 * x + 1;
        if ((new_x2 + y2) - radius2).abs() < ((x2 + y2) - radius2).abs() {
            x2 = new_x2;
            x -= 1;
            println!("{} {}",x, y);
        }
    }
}
