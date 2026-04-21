use std::{fs::File, io::BufWriter, sync::Arc};

use raytracing::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Material configuration
    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left   = Dielectric::new(1.5);
    let material_bubble = Dielectric::new(1.0 / 1.5);
    let material_right       = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        Arc::new(material_ground),
    )));
    world.add(Box::new(Sphere::new(
        Point::new(0.0, 0.0, -1.2),
        0.5,
        Arc::new(material_center),
    )));
    world.add(Box::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        Arc::new(material_left),
    )));
    world.add(Box::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.4,
        Arc::new(material_bubble),
    )));
    world.add(Box::new(Sphere::new(
        Point::new(1.0, 0.0, -1.0),
        0.5,
        Arc::new(material_right),
    )));

    // Image configuration
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 1280;
    let sample_per_pixel: u32 = 100;
    let max_depth: u32 = 50;

    // Camera + Render
    let mut camera = Camera::new(aspect_ratio, image_width, sample_per_pixel, max_depth);
    camera.render(&world);

    // Save
    let file = File::create("example.ppm")?;
    let mut writer = BufWriter::new(file);
    camera.save(&mut writer)?;

    Ok(())
}
