use std::fs::File;
use std::io::prelude::*;

mod math;
use math::{Vec3, Ray, Sphere, HittableList, Hittable, HitRecord};


fn ray_color(r: &Ray, world: &dyn Hittable) -> Vec3 {
    let mut rec = HitRecord::new();
    if world.hit(r, 0., f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Vec3(1., 1., 1.));
    }
    let unit = r.dir.unit_vector();
    let t = 0.5*(unit.1 + 1.);
    (1. - t)*Vec3(1., 1., 1.) + t*Vec3(0.5, 0.7, 1.)
}

fn main() -> std::io::Result<()> {

    println!("Saving image in ppm format.");

    let image_width = 200;
    let image_height = 100;

    let mut file = File::create("image.ppm")?;
    file.write_all(b"P3\n")?;
    file.write_all(format!("{} {}\n255\n", image_width, image_height).as_bytes())?;

    let lower_left_corner = Vec3(-2., -1., -1.);
    let horizontal = Vec3(4., 0., 0.);
    let vertical = Vec3(0., 2., 0.);
    let origin = Vec3(0., 0., 0.);

    let mut world = HittableList::new();
    world.add(Box::new( Sphere { center: Vec3(0., 0., -1.), radius: 0.5 } ));
    world.add(Box::new( Sphere { center: Vec3(0., -100.5, -1.), radius: 100. } ));

    let mut pixel_vals = String::from("");
    for j in (0..image_height).rev() {
        println!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / image_width as f64;
            let v = j as f64 / image_height as f64;
            let r = ray![origin, lower_left_corner + u*horizontal + v*vertical];
            let color = ray_color(&r, &world);
            //file.write_all(color.write_color().as_bytes())?;
            pixel_vals.push_str(&color.write_color());
        }
    }
    file.write_all(pixel_vals.as_bytes())?;
    Ok(())
}
