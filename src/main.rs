use std::{fs::File, io::{BufWriter, stdout}};

use raytracing::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Image configuration
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 1920;
    
    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    // Camera + Render
    let mut camera = Camera::new(aspect_ratio, image_width);
    camera.render(&world);

    let file = File::create("example.ppm")?;
    let mut writer = BufWriter::new(file);
    camera.save(&mut writer)?;

    Ok(())
}
