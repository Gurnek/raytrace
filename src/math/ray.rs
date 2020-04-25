use super::vec::Vec3;

#[derive(Debug)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + t*self.dir
    }
}

#[macro_export]
macro_rules! ray {
    [$o:expr, $d:expr] => {
        Ray { orig: $o, dir: $d }
    }
}