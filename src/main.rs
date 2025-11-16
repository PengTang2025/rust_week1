//use ascii symbols to draw a circle in the terminal
fn main() {
    let radius = 10;
    let diameter = radius * 2;

    for y in 0..=diameter {
        for x in 0..=diameter {
            let dx = x as f32 - radius as f32;
            let dy = y as f32 - radius as f32;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance > radius as f32 - 0.5 && distance < radius as f32 + 0.5 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
