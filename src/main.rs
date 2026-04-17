mod types;

use std::io::{self, Write};
use std::fs::File;
use types::color::Color;
use types::point::Point;

fn main() -> std::io::Result<()>{
    let mut file: File = File::create("example.ppm")?;
    const IMAGE_WIDTH: u32 = 720;
    const IMAGE_RATIO: f32 = 16.0/9.0;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / IMAGE_RATIO) as u32;

    let buffer_header: String = format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    file.write_all(buffer_header.as_bytes())?;

    for j in 0..IMAGE_HEIGHT {
        print!("\rRemaninig lines: {:<5}\t", IMAGE_HEIGHT - j);
        io::stdout().flush()?;
        for i in 0..IMAGE_WIDTH {
            let col = Color::new(
                i as f32 / (IMAGE_WIDTH - 1) as f32,
                j as f32 / (IMAGE_HEIGHT - 1) as f32,
                0.0
            );
            
            file.write_all(col.write_color().as_bytes())?;
        } 
    }
    println!("\nDONE!");
    
    let prova1 = Point::new(1.0,2.0,3.0);
    let prova2 = Point::new(1.0,2.0,3.0);
    let v3 = prova1 + prova2;
    print!("{}", v3.len_square());

    Ok(())
}
