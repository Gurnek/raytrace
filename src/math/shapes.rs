use super::{Vec3, Ray};
use crate::materials::{Material, Lambertian};

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub mat: Box<dyn Material>
}

impl HitRecord { 
    fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.dir * *outward_normal < 0.;
        self.normal = if self.front_face {*outward_normal} else {-*outward_normal};
    }

    pub fn new() -> HitRecord {
        HitRecord { t: 0., p: Vec3(0., 0., 0.), normal: Vec3(0., 0., 0.), front_face: false, mat: Box::new(Lambertian::new()) }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Box<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.orig - self.center;
        let a = r.dir * r.dir;
        let half_b = oc * r.dir;
        let c = oc * oc - self.radius.powi(2);
        let discriminant = half_b*half_b - a*c;

        if discriminant > 0. {
            let root = discriminant.sqrt();
            let mut temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(temp);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                rec.mat = self.material.clone();
                return true
            }
            temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                rec.mat = self.material.clone();
                return true
            }
        }
        false
    }
}


pub struct HittableList {
    objs: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objs.push(object);
    }

    pub fn new() -> HittableList {
        HittableList { objs: vec![] }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest = t_max;

        for obj in self.objs.iter() {
            let mut temp = HitRecord::new();
            if (*obj).hit(r, t_min, closest, &mut temp) {
                hit_anything = true;
                closest = temp.t;
                *rec = temp;
            }
        }

        hit_anything
    }
}