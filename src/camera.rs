use std::io::{self, Write};
use std::time::Instant;

use crate::objects::hittables::{Hittable, HittableList};
use crate::random_f64;
use crate::types::color::Color;
use crate::types::interval::POSITIVE;
use crate::types::point::Point;
use crate::types::ray::Ray;
use crate::types::vector::Vec3;

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub sample_per_pixel: u32,
    pub max_depth: u32,
    image_height: u32,
    pixel_sample_scale: f64,
    center: Point,
    pixel00_loc: Point,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixels: Vec<Color>,
    verbose: bool,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32, sample_per_pixel: u32, max_depth: u32) -> Self {
        let verbose = true;

        let image_height = (image_width as f64 / aspect_ratio) as u32;
        let pixel_sample_scale = 1.0 / sample_per_pixel as f64;
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
            sample_per_pixel,
            max_depth,
            image_height,
            pixel_sample_scale,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            pixels,
            verbose,
        }
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    pub fn render(&mut self, world: &HittableList) {
        self.pixels.clear();

        if self.verbose {
            println!(
                "Starting render: {}x{} pixels (SPP: {})",
                self.image_width, self.image_height, self.sample_per_pixel
            );
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
        writeln!(
            writer,
            "P3\n{} {}\n255",
            self.image_width, self.image_height
        )
    }

    fn render_pixel(&self, i: u32, j: u32, world: &HittableList) -> Color {
        let mut pixel_color = Color::default();

        for _ in 0..self.sample_per_pixel {
            let mut r = self.get_ray(i, j);
            pixel_color += Self::ray_color(&mut r, world, self.max_depth);
        }

        pixel_color * self.pixel_sample_scale
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);
        let ray_direction: Vec3 = pixel_sample - self.center;
        Ray::new(self.center, ray_direction)
    }

    #[inline] fn sample_square() -> Vec3 {
        Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.0)
    }

    fn ray_color(r: &mut Ray, world: &HittableList, max_depth: u32) -> Color {
        let mut attenuation = 1.0;

        for _ in 0..max_depth {
            if let Some(rec) = world.hit(r, POSITIVE) {
                r.set_orig(rec.p); 
                r.set_dir(rec.normal + Vec3::random_unit_vector()); 
                attenuation *= 0.7;
            } else {
                let unit_dir = r.dir().unit_vector();
                let a = 0.5 * (unit_dir.y + 1.0);
                let sky = (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
                return sky * attenuation; 
            }
        }
            
        Color::default()
    }

    fn report_progress(&self, j: u32, start_time: &Instant) {
        let elapsed = start_time.elapsed().as_secs_f64();
        let progress = (j + 1) as f64 / self.image_height as f64;
        let pixels_processed = (j + 1) * self.image_width;
        let rays_processed = pixels_processed * self.sample_per_pixel;
        let rays_per_sec = if elapsed > 0.0 {
            rays_processed as f64 / elapsed
        } else {
            0.0
        };

        print!(
            "\rProgress: {:.2}% | Elapsed: {:.1}s | Speed: {:.0} rays/s\x1b[K",
            progress * 100.0,
            elapsed,
            rays_per_sec
        );
        let _ = io::stdout().flush();
    }

    fn print_final_summary(&self, start_time: &Instant) {
        let duration = start_time.elapsed();
        let total_pixels = self.image_width * self.image_height;
        let total_rays = total_pixels * self.sample_per_pixel;
        println!("\n\nRender Complete!");
        println!("Total Time: {:.3} seconds", duration.as_secs_f64());
        println!("Total Pixels: {}", total_pixels);
        println!("Samples Per Pixel: {}", self.sample_per_pixel);
        println!("Total Rays: {}", total_rays);
        println!(
            "Average Speed: {:.0} rays per second",
            total_rays as f64 / duration.as_secs_f64()
        );
    }
}
