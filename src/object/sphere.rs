use crate::object::{Hit, Object};
use crate::ray::Ray;
use crate::vec3::Vec3;
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}
impl Sphere{
    pub fn new(center:Vec3,radius:f32)->Sphere{
        Sphere{
            center,
            radius
        }
    }
}
impl Object for Sphere {
    fn hits(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc = ray.src - self.center;
        let a = ray.dir.dot(ray.dir);
        let b = 2.0 * oc.dot(ray.dir);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discr = b * b - 4.0 * a * c;
        if discr >= 0.0 {
            let t = (-b - discr.sqrt()) / (2.0 * a);
            if t > t_min && t < t_max {
                return Some(Hit{
                    t,
                    p:ray.at_parameter(t),
                    normal:(ray.at_parameter(t) - self.center)/self.radius,
                })
            }
            let t = (-b + discr.sqrt()) / (2.0 * a);
            if t > t_min && t < t_max {
                return Some(Hit{
                    t,
                    p:ray.at_parameter(t),
                    normal:(ray.at_parameter(t) - self.center)/self.radius,
                })
            }
        }
        None
    }
}