use std::fs::File;
use std::io::prelude::*;

mod vec;
use vec::Vec3;

fn main() -> std::io::Result<()> {

    println!("Saving image in ppm format.");

    let image_width = 1000;
    let image_height = 1000;

    let mut file = File::create("image.ppm")?;
    file.write_all(b"P3\n")?;
    file.write_all(format!("{} {}\n255\n", image_height, image_width).as_bytes())?;
    for j in (0..image_height).rev() {
        println!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let v = Vec3(i as f64 / image_width as f64, j as f64 / image_height as f64, 0.2);
            file.write_all(v.write_color().as_bytes())?;
        }
    }

    Ok(())
}
