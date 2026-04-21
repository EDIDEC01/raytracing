use std::io::{self, Write};
use std::time::Instant;

use crate::{
    Color, Hittable, HittableList, POSITIVE, Point, Ray, Vec3, degrees_to_radians, random_f64,
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub sample_per_pixel: u32,
    pub max_depth: u32,
    pub vfov: f64,
    pub look_from: Point,
    pub look_at: Point,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    pub verbose: bool,

    image_height: u32,
    pixel_sample_scale: f64,
    center: Point,
    pixel00_loc: Point,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    pixels: Vec<Color>,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            sample_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            look_from: Point::default(),
            look_at: Point::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            verbose: true,
            image_height: 0,
            pixel_sample_scale: 0.0,
            center: Point::default(),
            pixel00_loc: Point::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
            defocus_disk_u: Vec3::default(),
            defocus_disk_v: Vec3::default(),
            pixels: Vec::new(),
        }
    }
}

impl Camera {
    fn init(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        self.pixel_sample_scale = 1.0 / self.sample_per_pixel as f64;
        self.center = self.look_from;

        let theta = degrees_to_radians(self.vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.w = Vec3::unit_vector(self.look_from - self.look_at);
        self.u = Vec3::unit_vector(Vec3::cross(self.vup, self.w));
        self.v = Vec3::cross(self.w, self.u);

        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);

        let viewport_upper_left = self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_dist * f64::tan(degrees_to_radians(self.defocus_angle / 2.0));
        self.defocus_disk_u = defocus_radius * self.u;
        self.defocus_disk_v = defocus_radius * self.v;

        self.pixels = Vec::with_capacity((self.image_width * self.image_height) as usize);
        self.verbose = true;
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    pub fn render(&mut self, world: &HittableList) {
        self.pixels.clear();
        self.init();

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
        let ray_origin = if self.defocus_angle <= 0.0 { self.center } else { self.defocus_disk_sample() };
        let ray_direction: Vec3 = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    #[inline]
    fn sample_square() -> Vec3 {
        Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Point {
        let p = Vec3::random_in_unit_disk();
        self.center + p.x * self.defocus_disk_u + p.y * self.defocus_disk_v
    }

    fn ray_color(r: &mut Ray, world: &HittableList, max_depth: u32) -> Color {
        let mut result = Color::new(1.0, 1.0, 1.0);

        for _ in 0..max_depth {
            if let Some(rec) = world.hit(r, POSITIVE) {
                if let Some((scattered, attenuation)) = rec.material.scatter(r, &rec) {
                    result = result * attenuation;
                    *r = scattered;
                } else {
                    return Color::default();
                }
            } else {
                let unit_dir = Vec3::unit_vector(r.dir());
                let a = 0.5 * (unit_dir.y + 1.0);
                let sky = (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
                return sky * result;
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
            "P6\n{} {}\n255",
            self.image_width, self.image_height
        )
    }
}
