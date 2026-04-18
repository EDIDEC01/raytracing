use std::fs::File;
use std::io::{self, BufWriter, Write};
use raytracing::*;

fn ray_color(r: &Ray, world: &HittableList) -> Color {
    if let Some(rec) = world.hit(r, Interval::new(0.0, INFINITY)) {
        return 0.5 * (Color::new(rec.normal.x + 1.0, rec.normal.y + 1.0, rec.normal.z +1.0));
    }

    let unit_dir = r.dir().unit_vector();
    let a = 0.5 * (unit_dir.y + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() -> io::Result<()> {
    let file = File::create("example.ppm")?;
    let mut writer = BufWriter::new(file);

    let image_width: u32 = 1920;
    let aspect_ratio: f64 = 16.0 / 9.0;
    let mut image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    image_height = if image_height < 1 { 1 } else { image_height };

    writeln!(writer, "P3\n{image_width} {image_height}\n255")?;

    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center: Point = Point::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / (image_width as f64);
    let pixel_delta_v = viewport_v / (image_height as f64);

    let viewport_upper_left: Point =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc: Point = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction: Vec3 = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r, &world);
            pixel_color.write_color(&mut writer)?;
        }
        if (j + 1) % 10 == 0 {
            print!(
                "\rCompletion percentage: {:.2}% ({}/{})",
                (j + 1) as f64 / image_height as f64 * 100.0,
                j + 1,
                image_height
            );
            io::stdout().flush()?;
        }
    }
    writer.flush()?;
    println!("\nDONE!");

    Ok(())
}
