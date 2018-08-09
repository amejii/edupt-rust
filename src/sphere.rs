use vector;
use vector::Vector;
use ray::Ray;
use material::{Color,ReflectionType};
use constant::K_EPS;
use intersection::Hitpoint;
use vecmath::traits::Sqrt;

#[derive(Debug)]
pub struct Sphere {
    pub radius: f64,
    pub position: Vector,
    pub emission: Color,
    pub color: Color,
    pub reflection_type: ReflectionType,
}

impl Sphere{
    fn new(radius:f64, position: Vector, emission: Color, color: Color, reflection_type: ReflectionType) -> Sphere {
        Sphere{
            radius: radius,
            position: position,
            emission: emission,
            color: color,
            reflection_type: reflection_type,
        }
    }

    pub fn intersect(&self, ray: &Ray, mut hitpoint: &mut Hitpoint) -> bool{
        let mut p_o = self.position - ray.org;
        let mut b = vector::dot(p_o, ray.dir);
        let mut D4 = b * b - vector::dot(p_o, p_o) + self.radius * self.radius;

        if(D4 < 0.0){
            return false;
        }

        let sqrt_D4 = Sqrt::sqrt(D4);
        let t1 = b - sqrt_D4;
        let t2 = b + sqrt_D4;

        if(t1 < K_EPS && t2 < K_EPS){
            return false;
        }

        if(t1 > K_EPS){
            hitpoint.distance = t1;
        }else{
            hitpoint.distance = t2;
        }

        hitpoint.position = ray.org + hitpoint.distance * ray.dir;
        hitpoint.normal = vector::normalize(hitpoint.position - self.position);

        true
    }
}