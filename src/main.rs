use std::{fs::File, io::BufWriter, sync::Arc};

use raytracing::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Material
    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.50);
    let material_bubble = Dielectric::new(1.00 / 1.50);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

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

    // Camera configuration + Render
    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.sample_per_pixel = 100;
    camera.max_depth = 50;
    camera.vfov = 20.0;
    camera.look_from = Point::new(-2.0, 2.0, 1.0);
    camera.look_at = Point::new(0.0, 0.0, -1.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 10.0;
    camera.focus_dist = 3.4;
    
    camera.render(&world);

    // Save
    let file = File::create("example.ppm")?;
    let mut writer = BufWriter::new(file);
    camera.save(&mut writer)?;

    Ok(())
}
