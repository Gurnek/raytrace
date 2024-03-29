use rayon::prelude::*;
use std::io::prelude::*;
use std::{fs::File, sync::Arc};

#[macro_use]
mod math;
use math::{HitRecord, Hittable, HittableList, Ray, Sphere, Vec3};

mod camera;
use camera::Camera;

mod materials;
use materials::{Dielectric, Lambertian, Metal};
use rand::Rng;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
    if depth == 0 {
        return Vec3(0., 0., 0.);
    }
    let mut rec = HitRecord::new();
    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        let mut scattered = Ray::new();
        let mut attenuation = Vec3(0., 0., 0.);
        if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
            let t = ray_color(&scattered, world, depth - 1);
            return Vec3(
                attenuation.0 * t.0,
                attenuation.1 * t.1,
                attenuation.2 * t.2,
            );
        } else {
            return Vec3(0., 0., 0.);
        }
    }
    let unit = r.dir.unit_vector();
    let t = 0.5 * (unit.1 + 1.);
    (1. - t) * Vec3(1., 1., 1.) + t * Vec3(0.5, 0.7, 1.)
}

fn main() -> std::io::Result<()> {
    println!("Saving image in ppm format.");

    let image_width = 600;
    let image_height = 400;
    let samples_per_pixel = 100;
    let max_depth = 25;

    let mut file = File::create("image.ppm")?;
    file.write_all(b"P3\n")?;
    file.write_all(format!("{} {}\n255\n", image_width, image_height).as_bytes())?;

    let mut world = HittableList::new();
    world.add(Box::new(Sphere {
        center: Vec3(0., 0., -1.),
        radius: 0.5,
        material: Arc::new(Lambertian {
            albedo: Vec3(0.7, 0.3, 0.3),
        }),
    }));
    world.add(Box::new(Sphere {
        center: Vec3(0., -100.5, -1.),
        radius: 100.,
        material: Arc::new(Lambertian {
            albedo: Vec3(0.8, 0.8, 0.),
        }),
    }));
    world.add(Box::new(Sphere {
        center: Vec3(1., 0., -1.),
        radius: 0.5,
        material: Arc::new(Metal::new(Vec3(0.8, 0.6, 0.2), 0.)),
    }));
    world.add(Box::new(Sphere {
        center: Vec3(-1., 0., -1.),
        radius: 0.5,
        material: Arc::new(Dielectric::new(1.5)),
    }));
    world.add(Box::new(Sphere {
        center: Vec3(-1., 0., -1.),
        radius: -0.45,
        material: Arc::new(Dielectric::new(1.5)),
    }));

    let lookfrom = Vec3(3., 3., 2.);
    let lookat = Vec3(0., 0., -1.);
    let vup = Vec3(0., 1., 0.);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 2.;
    let aspect_ratio = image_width as f64 / image_height as f64;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        std::f64::consts::PI / 9.,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let pixel_vals = (0..image_height)
        .into_par_iter()
        .rev()
        .map(|j| {
            println!("\rScanlines remaining: {}", j);
            (0..image_width)
                .map(|i| {
                    let color: Vec3 = (0..samples_per_pixel)
                        .map(|_| {
                            let mut rng = rand::thread_rng();
                            let u = ((i as f64) + rng.gen::<f64>()) / image_width as f64;
                            let v = ((j as f64) + rng.gen::<f64>()) / image_height as f64;
                            let r = cam.get_ray(u, v);
                            ray_color(&r, &world, max_depth)
                        })
                        .fold(Vec3(0., 0., 0.), |acc, x| acc + x);
                    color.write_color(samples_per_pixel)
                })
                .fold(String::new(), |a, s| a + &s)
        })
        .fold(|| String::new(), |a, s| a + &s)
        .reduce(|| String::new(), |a, s| a + &s);
    file.write_all(pixel_vals.as_bytes())?;
    Ok(())
}
