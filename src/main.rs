mod camera;
mod ray;
mod utils;
mod vec3;
mod world;

use crate::{
    utils::helpers::random_f32,
    vec3::{
        color3::Color3,
        vec3::{Point3, Vec3},
    },
    world::material::Material,
};
use camera::camera::Camera;
use rand::Rng;
use std::{
    fs::File,
    io::{self, Write},
    sync::Arc,
    thread,
};
use world::{scene_object::SceneObject, scene_object_list::SceneObjectList};

fn main() {
    let mut world = SceneObjectList::new();
    let mut rng = rand::thread_rng();

    // Ground
    let ground_material = Arc::new(Material::Lambertian {
        albedo: Color3::new(0.5, 0.5, 0.5),
    });
    world.add(SceneObject::Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material.clone(),
    });

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Vec3::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color3::random() * Color3::random();
                    Arc::new(Material::Lambertian { albedo })
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color3::random_range(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    Arc::new(Material::Metal { albedo, fuzz })
                } else {
                    // glass
                    Arc::new(Material::Dielectric {
                        refraction_index: 1.5,
                    })
                };

                if random_f32() > 0.5 {
                    world.add(SceneObject::Sphere {
                        center,
                        radius: 0.2,
                        material: material,
                    });
                } else {
                    world.add(SceneObject::Cylinder {
                        radius: 0.2,
                        center: center,
                        height: 0.5,
                        material: material,
                    });
                }
            }
        }
    }

    // Three main spheres
    let material1 = Arc::new(Material::Dielectric {
        refraction_index: 1.5,
    });
    let material2 = Arc::new(Material::Lambertian {
        albedo: Color3::new(0.4, 0.2, 0.1),
    });
    let material3 = Arc::new(Material::Metal {
        albedo: Color3::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });

    world.add(SceneObject::Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: material1,
    });
    world.add(SceneObject::Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: material2,
    });
    world.add(SceneObject::Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: material3,
    });
    // Camera configuration
    let cam = Camera::new(
        1200,
        16.0 / 9.0,
        500,
        25.0,
        20,
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.6,
        10.0,
    );

    let image_height: usize = (cam.image_width as f32 / cam.aspect_ratio) as usize;

    let num_threads = 7;
    println!(
        "Rendering {}x{} image with {} threads...",
        cam.image_width, image_height, num_threads
    );

    let rows_per_thread = image_height / num_threads;
    let mut handles = vec![];
    let mut subimages: Vec<Vec<Color3>> =
        vec![
            vec![Color3::new(0.0, 0.0, 0.0); rows_per_thread * cam.image_width as usize];
            num_threads
        ];

    subimages[num_threads - 1].resize(
        (image_height - (num_threads - 1) * rows_per_thread) * cam.image_width as usize,
        Color3::new(0.0, 0.0, 0.0),
    );

    let shared_world = Arc::new(world);

    for (i, mut subimage) in subimages.into_iter().enumerate() {
        let start_row = i * rows_per_thread;
        let end_row = if i == num_threads - 1 {
            image_height // Last thread handles remaining rows
        } else {
            (i + 1) * rows_per_thread
        };

        let cam_clone = cam.clone();
        let world_clone = Arc::clone(&shared_world);

        let handle = thread::spawn(move || {
            cam_clone.render(
                world_clone,
                (0, cam_clone.image_width as usize),
                (start_row, end_row),
                &mut subimage,
            );
            subimage
        });
        handles.push(handle);
    }

    // Collect results from all threads
    let mut image: Vec<Vec<Color3>> = vec![];
    for handle in handles {
        image.push(handle.join().unwrap());
    }

    write_image_to_file(&cam, image_height, &image).expect("Failed to write image file");
}

fn write_image_to_file(
    cam: &Camera,
    image_height: usize,
    image: &Vec<Vec<Color3>>,
) -> io::Result<()> {
    let mut file = File::create("image.ppm")?;

    // Write PPM header
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", cam.image_width, image_height)?;
    writeln!(file, "255")?;

    for subimage in image.iter() {
        for color in subimage {
            let r = (255.0 * color.x.clamp(0.0, 1.0)) as u8;
            let g = (255.0 * color.y.clamp(0.0, 1.0)) as u8;
            let b = (255.0 * color.z.clamp(0.0, 1.0)) as u8;

            writeln!(file, "{} {} {}", r, g, b)?;
        }
    }

    Ok(())
}
