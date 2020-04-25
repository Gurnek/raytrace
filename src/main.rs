use std::fs::File;
use std::io::prelude::*;

mod math;
use math::Vec3;
use math::Ray;

fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> f64 {
    let oc = r.orig - *center;
    let a = r.dir * r.dir;
    let half_b = oc * r.dir;
    let c = oc * oc - radius.powi(2);
    let discriminant = half_b*half_b - a*c;
    if discriminant < 0. {
        -1.
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(r: &Ray) -> Vec3 {
    let mut t = hit_sphere(&Vec3(0., 0., -1.), 0.5, r);
    if t > 0. {
        let N = (r.at(t) - Vec3(0., 0., -1.)).unit_vector();
        return 0.5*Vec3(N.0+1., N.1+1., N.2+1.)
    }
    let unit = r.dir.unit_vector();
    t = 0.5*(unit.1 + 1.);
    (1. - t)*Vec3(1., 1., 1.) + t*Vec3(0.5, 0.7, 1.)
}

fn main() -> std::io::Result<()> {

    println!("Saving image in ppm format.");

    let image_width = 2000;
    let image_height = 1000;

    let mut file = File::create("image.ppm")?;
    file.write_all(b"P3\n")?;
    file.write_all(format!("{} {}\n255\n", image_width, image_height).as_bytes())?;

    let lower_left_corner = Vec3(-2., -1., -1.);
    let horizontal = Vec3(4., 0., 0.);
    let vertical = Vec3(0., 2., 0.);
    let origin = Vec3(0., 0., 0.);

    let mut pixel_vals = String::from("");
    for j in (0..image_height).rev() {
        println!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / image_width as f64;
            let v = j as f64 / image_height as f64;
            let r = ray![origin, lower_left_corner + u*horizontal + v*vertical];
            let color = ray_color(&r);
            //file.write_all(color.write_color().as_bytes())?;
            pixel_vals.push_str(&color.write_color());
        }
    }
    file.write_all(pixel_vals.as_bytes())?;
    Ok(())
}
