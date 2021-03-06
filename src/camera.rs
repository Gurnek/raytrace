use crate::math::{Vec3, Ray};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    w: Vec3, 
    u: Vec3,
    v: Vec3,
    lens_radius: f64
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> Camera {
        let h = (vfov / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;
        
        let w: Vec3 = (lookfrom - lookat).unit_vector();
        let u = (vup.cross(&w)).unit_vector();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lens_radius = aperture / 2.;

        Camera {
            lower_left_corner: origin - horizontal/2. - vertical/2. - focus_dist*w,
            horizontal: horizontal,
            vertical: vertical,
            origin: origin,
            w: w,
            u: u,
            v: v,
            lens_radius: lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.0 + self.v * rd.1;

        ray![self.origin + offset, self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset]
    }
}