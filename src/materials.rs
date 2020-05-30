use crate::math::{Ray, HitRecord, Vec3};
use rand::prelude::random;

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

#[derive(Clone)]
pub struct Dielectric {
    pub ref_idx: f64,
}

impl Dielectric {
    fn refract(&self, uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-*uv) * *n;
        let r_out_parallel: Vec3 = etai_over_etat * (*uv + cos_theta * *n);
        let r_out_perp = -*n * ((1. - r_out_parallel.length_squared()).sqrt());
        r_out_parallel + r_out_perp
    }

    fn reflect(&self, v: &Vec3, n: &Vec3) -> Vec3 {
        *v - *n * (2.*(*v * *n))
    }

    pub fn new(ref_idx: f64) -> Dielectric {
        Dielectric { ref_idx: ref_idx }
    }

    fn schlick(&self, cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1. - ref_idx) / (1. + ref_idx);
        r0 = r0 * r0;
        r0 + (1. - r0) * (1. - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        *attenuation = Vec3(1., 1., 1.);
        let etai_over_etat = if rec.front_face {1. / self.ref_idx} else {self.ref_idx};

        let unit_dir = r_in.dir.unit_vector();
        let cos_theta = 1_f64.min((-unit_dir) * rec.normal);
        let sin_theta = (1. - cos_theta*cos_theta).sqrt();
        
        if etai_over_etat * sin_theta > 1. {
            let reflected = self.reflect(&unit_dir, &rec.normal);
            *scattered = ray![rec.p, reflected];
            return true
        }

        let reflect_prob = self.schlick(cos_theta, etai_over_etat);
        if random::<f64>() < reflect_prob {
            let reflected = self.reflect(&unit_dir, &rec.normal);
            *scattered = ray![rec.p, reflected];
            return true;
        }
        let refracted = self.refract(&unit_dir, &rec.normal, etai_over_etat);
        *scattered = ray![rec.p, refracted];
        true
    }
}