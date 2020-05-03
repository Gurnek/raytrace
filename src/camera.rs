use crate::math::{Vec3, Ray};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            lower_left_corner: Vec3(-2., -1., -1.),
            horizontal: Vec3(4., 0., 0.),
            vertical: Vec3(0., 2., 0.),
            origin: Vec3(0., 0., 0.)
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        ray![self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical]
    }
}