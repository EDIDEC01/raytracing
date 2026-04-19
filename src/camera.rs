use std::io::{self, Write};
use std::time::Instant;

use crate::objects::hittables::{HittableList, Hittable};
use crate::types::color::Color;
use crate::types::interval::POSITIVE;
use crate::types::point::Point;
use crate::types::ray::Ray;
use crate::types::vector::Vec3;

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    verbose: bool,
    image_height: u32,
    center: Point,
    pixel00_loc: Point,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixels: Vec<Color>,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32) -> Self {
        let verbose = true;

        let image_height = (image_width as f64 / aspect_ratio) as u32;
        let center = Point::default();

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
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let pixels = Vec::with_capacity((image_width * image_height) as usize);

        Self {
            aspect_ratio,
            image_width,
            verbose,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            pixels,
        }
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    pub fn render(&mut self, world: &HittableList) {
        self.pixels.clear();
        
        if self.verbose {
            println!("Starting render: {}x{} pixels", self.image_width, self.image_height);
        }
        let start_time = Instant::now();

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let pixel_color = self.render_pixel(i, j, world);
                self.pixels.push(pixel_color);
            }
            
            if self.verbose {
                self.report_progress(j, &start_time);
            }
        }
        
        if self.verbose {
            self.print_final_summary(&start_time);
        }
    }

    pub fn save(&self, writer: &mut impl Write) -> io::Result<()> {
        if self.pixels.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "No pixels to save. Call render() before calling save().",
            ));
        }

        self.write_ppm_header(writer)?;

        for pixel in &self.pixels {
            pixel.write_color(writer)?;
        }

        writer.flush()
    }

    fn write_ppm_header(&self, writer: &mut impl Write) -> io::Result<()> {
        writeln!(writer, "P3\n{} {}\n255", self.image_width, self.image_height)
    }
    
    fn render_pixel(&self, i: u32, j: u32, world: &HittableList) -> Color {
        let pixel_center =
        self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let ray_direction: Vec3 = pixel_center - self.center;
        let r = Ray::new(self.center, ray_direction);
        
        Self::ray_color(&r, &world)
    }
    
    fn ray_color(r: &Ray, world: &HittableList) -> Color {
        if let Some(rec) = world.hit(r, POSITIVE) {
            return 0.5 * Color::new(rec.normal.x + 1.0, rec.normal.y + 1.0, rec.normal.z + 1.0);
        }
        
        let unit_dir = r.dir().unit_vector();
        let a = 0.5 * (unit_dir.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    fn report_progress(&self, j: u32, start_time: &Instant) {
        if (j + 1) % 10 != 0 && j != self.image_height - 1 {
            return;
        }
        let elapsed = start_time.elapsed().as_secs_f64();
        let progress = (j + 1) as f64 / self.image_height as f64;
        let pixels_processed = (j + 1) * self.image_width;
        let pixels_per_sec = if elapsed > 0.0 { pixels_processed as f64 / elapsed } else { 0.0 };
        
        print!(
            "\rProgress: {:.2}% | Elapsed: {:.1}s | Speed: {:.0} px/s",
            progress * 100.0,
            elapsed,
            pixels_per_sec
        );
        let _ = io::stdout().flush();
    }

    fn print_final_summary(&self, start_time: &Instant) {
        let duration = start_time.elapsed();
        println!("\n\nRender Complete!");
        println!("Total Time: {:.3} seconds", duration.as_secs_f64());
        println!("Total Pixels: {}", self.image_width * self.image_height);
        println!("Average Speed: {:.0} pixels per second", (self.image_width * self.image_height) as f64 / duration.as_secs_f64());
    }
}