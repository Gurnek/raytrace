use crate::math::{Vec3, Ray};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Camera {
        let h = (vfov / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;
        
        let w: Vec3 = (lookfrom - lookat).unit_vector();
        let u = (vup.cross(&w)).unit_vector();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;

        Camera {
            lower_left_corner: origin - horizontal/2. - vertical/2. - w,
            horizontal: horizontal,
            vertical: vertical,
            origin: origin
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        ray![self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin]
    }
}