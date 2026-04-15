use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>{
    let mut file: File = File::create("example.ppm")?;
    const IMAGE_WIDTH: u32 = 1920;
    const IMAGE_RATIO: f32 = 16.0/9.0;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / IMAGE_RATIO) as u32;

    let buffer_header: String = format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    file.write_all(buffer_header.as_bytes())?;

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.0;

            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;
            file.write_all(format!("{ir} {ig} {ib}\n").as_bytes())?;
        } 
    }
    Ok(())

}
