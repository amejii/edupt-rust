use constant::K_INF;
use sphere::Sphere;
use intersection::{Hitpoint,Intersection};
use vector;
use vector::Vector;
use ray::Ray;
use material::{Color,ReflectionType};
use vecmath::traits::Sqrt;

pub const spheres : [Sphere;10]= [
    Sphere{radius:1e5, position:Vector{x:1e5+1.0, y:40.8, z:81.6}, emission:Color{x:0.0,y:0.0,z:0.0},  color:Color{x:0.75, y:0.25, z:0.25}, reflection_type:ReflectionType::REFLECTION_TYPE_DIFFUSE},// left
	Sphere{radius:1e5, position:Vector{x:-1e5+99.0, y:40.8, z:81.6},emission:Color{x:0.0,y:0.0,z:0.0},  color:Color{x:0.25, y:0.25, z:0.75}, reflection_type:ReflectionType::REFLECTION_TYPE_DIFFUSE}, // right
	Sphere{radius:1e5, position:Vector{x:50.0, y:40.8, z:1e5},      emission:Color{x:0.0,y:0.0,z:0.0},  color:Color{x:0.75, y:0.75, z:0.75}, reflection_type:ReflectionType::REFLECTION_TYPE_DIFFUSE}, // back
	Sphere{radius:1e5, position:Vector{x:50.0, y:40.8, z:-1e5+250.0}, emission:Color{x:0.0,y:0.0,z:0.0},  color:Color{x:0.0,y:0.0,z:0.0},      reflection_type:ReflectionType::REFLECTION_TYPE_DIFFUSE}, // front
	Sphere{radius:1e5, position:Vector{x:50.0, y:1e5, z:81.6},      emission:Color{x:0.0,y:0.0,z:0.0},  color:Color{x:0.75, y:0.75, z:0.75}, reflection_type:ReflectionType::REFLECTION_TYPE_DIFFUSE}, // floor
	Sphere{radius:1e5, position:Vector{x:50.0, y:-1e5+81.6, z:81.6},emission:Color{x:0.0,y:0.0,z:0.0},  color:Color{x:0.75, y:0.75, z:0.75}, reflection_type:ReflectionType::REFLECTION_TYPE_DIFFUSE}, // roof
	Sphere{radius:20.0,  position:Vector{x:65.0, y:20.0, z:20.0},         emission:Color{x:0.0,y:0.0,z:0.0},  color:Color{x:0.25, y:0.75, z:0.25}, reflection_type:ReflectionType::REFLECTION_TYPE_DIFFUSE}, // green-sphere
	Sphere{radius:16.5,position:Vector{x:27.0, y:16.5, z:47.0},       emission:Color{x:0.0,y:0.0,z:0.0},  color:Color{x:0.99, y:0.99, z:0.99}, reflection_type:ReflectionType::REFLECTION_TYPE_SPECULAR}, // mirror
	Sphere{radius:16.5,position:Vector{x:77.0, y:16.5, z:78.0},       emission:Color{x:0.0,y:0.0,z:0.0},  color:Color{x:0.99, y:0.99, z:0.99}, reflection_type:ReflectionType::REFLECTION_TYPE_REFRACTION}, //glass
	Sphere{radius:15.0,position:Vector{x:50.0, y:90.0, z:81.6},   emission:Color{x:36.0,y:36.0,z:36.0},     color:Color{x:0.0,y:0.0,z:0.0},      reflection_type:ReflectionType::REFLECTION_TYPE_DIFFUSE}, //illumination
];

pub fn intersect_scene(ray: &Ray, mut intersection: &mut Intersection) -> bool{
    let n = spheres.len();

    intersection.hitpoint.distance = K_INF;
    intersection.object_id = -1;

    for i in 0..=n-1 {
        let mut hitpoint : Hitpoint = Hitpoint::new();
        if (spheres[i].intersect(ray, &mut hitpoint)){
            if(hitpoint.distance < intersection.hitpoint.distance){
                intersection.hitpoint = hitpoint;
                intersection.object_id = i as i32;
            }
        }
    }

    intersection.object_id != -1
}