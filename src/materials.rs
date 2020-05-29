use crate::math::{Ray, HitRecord, Vec3};

pub trait Material : MaterialClone {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

pub trait MaterialClone {
    fn clone_box(&self) -> Box<dyn Material>;
}

impl<T> MaterialClone for T
where
    T: 'static + Material + Clone
{
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new() -> Lambertian {
        Lambertian { albedo: Vec3(0.7, 0.3, 0.5) }
    }
}

impl Material for Lambertian {
    fn scatter<'a>(&'a self, _r_in: &Ray, rec: &HitRecord, attenuation: &'a mut Vec3, scattered: &mut Ray) -> bool {
        let scatter_direction = rec.normal + Vec3::random_unit_vector();
        *scattered = ray![rec.p, scatter_direction];
        *attenuation = self.albedo;
        true
    }
}

#[derive(Clone)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    fn reflect(&self, v: &Vec3, n: &Vec3) -> Vec3 {
        *v - *n*(2.*(*v**n))
    }

    pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
        let fuzz = if fuzz < 1. {fuzz} else {1.};
        Metal { albedo: albedo, fuzz: fuzz}
    }
}

impl Material for Metal {
    fn scatter<'a>(&'a self, r_in: &Ray, rec: &HitRecord, attenuation: &'a mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = self.reflect(&r_in.dir.unit_vector(), &rec.normal);
        *scattered = ray![rec.p, reflected + self.fuzz*Vec3::random_in_unit_sphere()];
        *attenuation = self.albedo;
        scattered.dir * rec.normal > 0.
    }
}