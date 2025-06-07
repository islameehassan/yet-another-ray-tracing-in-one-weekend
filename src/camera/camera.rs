use std::io::{self};
use std::sync::{Arc, Mutex};

use crate::utils::helpers::degress_to_radians;
use crate::world::hit_record::HitRecord;
use crate::{
    ray::ray::Ray,
    utils::{constants::INFINITY, helpers::random_f32, interval::Interval},
};
use crate::{
    vec3::{
        color3::{write_color, Color3},
        vec3::{Point3, Vec3},
    },
    world::scene_object_list::SceneObjectList,
};

#[derive(Debug, Clone)]
pub struct Camera {
    pub image_width: u32,
    pub aspect_ratio: f32,
    pub samples_per_pixel: u32,
    pub vfov: f32, // in degrees
    pub max_depth: u32,
    pub camera_position: Point3,
    pub lookat: Point3,
    pub upvector: Vec3,
    pub defocus_angle: f32,
    pub focus_dist: f32,

    pub image_height: u32,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_origin: Point3,
    center: Point3,
    pixel_sample_scale: f32,
    gamma_correction: f32,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        image_width: u32,
        aspect_ratio: f32,
        samples_per_pixel: u32,
        vfov: f32,
        max_depth: u32,
        camera_position: Point3,
        lookat: Point3,
        upvector: Vec3,
        defocus_angle: f32,
        focus_dist: f32,
    ) -> Self {
        let pixel_sample_scale = 1.0 / (samples_per_pixel as f32);

        let mut image_height = (image_width as f32 / aspect_ratio) as u32;
        if image_height < 1 {
            image_height = 1;
        }

        let center = camera_position;

        // viewport
        let vfov_radians = degress_to_radians(vfov);
        let h = f32::tan(vfov_radians / 2.0);

        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

        // camera coordinate system
        let w = Vec3::unit(camera_position - lookat);
        let u = Vec3::unit(Vec3::cross_product(upvector, w));
        let v = Vec3::cross_product(w, u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -1.0 * v;

        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        let viewport_origin = center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel00_origin = viewport_origin + 0.5 * (pixel_delta_u + pixel_delta_u);

        let defocus_radius = focus_dist * f32::tan(degress_to_radians(defocus_angle / 2.0));
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            image_width,
            aspect_ratio,
            samples_per_pixel,
            vfov,
            max_depth,
            camera_position,
            lookat,
            upvector,
            defocus_angle,
            focus_dist,
            image_height,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_origin,
            center,
            pixel_sample_scale,
            gamma_correction: 0.0, // Initialize with default value
            u,
            v,
            w,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(
        &self,
        world: Arc<SceneObjectList>,
        x_range: (usize, usize),
        y_range: (usize, usize),
        pixels: &mut Vec<Color3>,
    ) {
        let total_pixels = (y_range.1 - y_range.0) * (x_range.1 - x_range.0);
        //let batch_size = total_pixels / 3;
        //let mut completed_pixels = 0;
        let thread_id = std::thread::current().id();

        println!(
            "Thread {:?} starting: {} pixels to render",
            thread_id, total_pixels
        );
        for y in y_range.0..y_range.1 {
            for x in x_range.0..x_range.1 {
                let mut pixel_color = Color3::new(0.0, 0.0, 0.0);


                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x as u32, y as u32);
                    pixel_color += self.ray_color(&ray, self.max_depth, &world);
                }

                pixel_color *= self.pixel_sample_scale;
                let gamma_corrected_color = Color3::new(
                    pixel_color.x.sqrt(),
                    pixel_color.y.sqrt(),
                    pixel_color.z.sqrt(),
                );

                let local_y = y - y_range.0; 
                let index = local_y * self.image_width as usize + x;
                pixels[index] = gamma_corrected_color;

                //completed_pixels += 1;
                
                /* 
                if completed_pixels % batch_size == 0 || completed_pixels == total_pixels {
                    let progress = (completed_pixels as f32 / total_pixels as f32) * 100.0;

                    println!(
                        "Thread {:?}: [{:>3.1}%] {:>6}/{:<6} pixels",
                        thread_id, progress, completed_pixels, total_pixels,
                    );
                }*/
            }
        }
    }

    #[inline]
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = Vec3::new(random_f32() - 0.5, random_f32() - 0.5, 0.0);
        let pixel_sample = self.pixel00_origin
            + (i as f32 + offset.x) * self.pixel_delta_u
            + (j as f32 + offset.y) * self.pixel_delta_v;
        let origin = if self.defocus_angle > 0.0 {
            self.defocus_disk_sample()
        } else {
            self.center
        };
        let direction = pixel_sample - origin;

        Ray::new(origin, direction)
    }

    #[inline]
    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }

    fn ray_color(&self, initial_ray: &Ray, depth: u32, world: &SceneObjectList) -> Color3 {
        let mut ray_origin = initial_ray.origin();
        let mut ray_direction = initial_ray.direction();
        let mut color = Color3::new(1.0, 1.0, 1.0);

        let mut hit_rec: HitRecord = HitRecord::default();
        let mut scattered = Ray::default();
        let mut attenuation = Color3::default();

        for _ in 0..depth {
            let current_ray = Ray::new(ray_origin, ray_direction);

            if world.hit(&current_ray, Interval::new(0.001, INFINITY), &mut hit_rec) {
                if hit_rec.material.scatter(
                    &current_ray,
                    &hit_rec,
                    &mut attenuation,
                    &mut scattered,
                ) {
                    color = color * attenuation;
                    // Update ray parameters for next iteration
                    ray_origin = scattered.origin();
                    ray_direction = scattered.direction();
                } else {
                    return Color3::new(0.0, 0.0, 0.0);
                }
            } else {
                // Hit sky
                let unit_vector = Vec3::unit(ray_direction);
                let a = 0.5 * (unit_vector.y + 1.0);
                let sky_color =
                    (1.0 - a) * Color3::new(1.0, 1.0, 1.0) + a * Color3::new(0.5, 0.7, 1.0);
                return color * sky_color;
            }
        }

        // Exhausted all bounces
        Color3::new(0.0, 0.0, 0.0)
    }
}
