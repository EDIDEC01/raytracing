mod types;

use std::fs::File;
use std::io::{self, Write};
use types::*;

fn hit_sphere(center: Point, radius: f32, r: &Ray) -> bool {
    let oc = center - r.orig();
    let a = dot(r.dir(), r.dir());
    let b = -2.0 * dot(r.dir(), oc);
    let c = dot(oc, oc) - f32::powi(radius, 2);
    let discriminant = b * b - 4.0 * a * c; 

    discriminant >= 0.0
}

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(Point::new(0.0,0.0,-1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let unit_dir = unit_vector(r.dir());
    let a = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() -> std::io::Result<()> {
    let mut file: File = File::create("example.ppm")?;
    let image_width: u32 = 1920;
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_height: u32 = (image_width as f32 / aspect_ratio) as u32;

    let buffer_header: String = format!("P3\n{image_width} {image_height}\n255\n");
    file.write_all(buffer_header.as_bytes())?;

    let focal_lenght: f32 = 1.0;
    let viewport_height: f32 = 2.0;
    let viewport_widht: f32 = viewport_height * (image_width as f32 / image_height as f32);
    let camera_center: Point = Point::new(0.0, 0.0, 0.0);

    let viewport_u: Vec3 = Vec3::new(viewport_widht, 0.0, 0.0);
    let viewport_v: Vec3 = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u: Vec3 = viewport_u / (image_width as f32);
    let pixel_delta_v: Vec3 = viewport_v / (image_height as f32);

    let viewport_upper_left: Point =
        camera_center - Vec3::new(0.0, 0.0, focal_lenght) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc: Point = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center: Point =
                pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            let ray_direction: Vec3 = pixel_center - camera_center;
            let r: Ray = Ray::new(pixel_center, ray_direction);

            let pixel_color: Color = ray_color(&r);
            file.write_all(pixel_color.write_color().as_bytes())?;
        }
        print!(
            "\rCompletion percentage: {:.2}% ({}/{})",
            (j + 1) as f32 / image_height as f32 * 100.0,
            j + 1,
            image_height
        );
        io::stdout().flush()?;
    }
    println!("\nDONE!");

    Ok(())
}
