use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    println!("Saving image in ppm format.");

    let image_width = 400;
    let image_height = 400;

    let mut file = File::create("image.ppm")?;
    file.write_all(b"P3\n")?;
    file.write_all(format!("{} {}\n255\n", image_height, image_width).as_bytes())?;
    for j in (0..image_height).rev() {
        println!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let r = ((i as f64 / image_width as f64) * 255.) as i32;
            let g = ((j as f64 / image_width as f64) * 255.) as i32;
            let b = (0.2 * 255.) as i32;
            file.write_all(format!("{} {} {}\n", r, g, b).as_bytes())?;
        }
    }

    Ok(())
}
