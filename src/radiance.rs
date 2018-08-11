use constant::{K_INF,K_EPS,K_PI};
use sphere::Sphere;
use intersection::{Hitpoint,Intersection};
use vector;
use vector::Vector;
use ray::Ray;
use material::{Color,ReflectionType};
use vecmath::traits::Sqrt;
use random::Random;
use scene::{intersect_scene,spheres};
use std;

const K_BACKGROUND_COLOR : Color = Color{x:0.0,y:0.0,z:0.0};
const K_DEPTH : i32 = 5;
const K_DEPTH_LIMIT : i32 = 64;

fn radiance(ray: Ray, mut rnd: Random, depth: i32) -> Color{
    let mut intersection = Intersection::new();
    
    if(!intersect_scene(&ray, &mut intersection)){
        return K_BACKGROUND_COLOR;
    }

    let now_object : &Sphere = &spheres[intersection.object_id as usize];
    let hitpoint : Hitpoint = intersection.hitpoint;
    let orienting_normal : Vector = if (vector::dot(hitpoint.normal, ray.dir) < 0.0){
        hitpoint.normal
    }else{
        (-1.0 * hitpoint.normal)
    };

    let mut russian_roulette_probability = now_object.color.x.max(now_object.color.y.max(now_object.color.z));

    if (depth > K_DEPTH){
        if(rnd.next01() >= russian_roulette_probability){
            return now_object.emission;
        }
    }else{
        russian_roulette_probability = 1.0;
    }

    let incoming_radiance : Color;
    let mut weight : Color = Color{x:1.0, y:1.0, z:1.0};

    match(now_object.reflection_type){
        ReflectionType::REFLECTION_TYPE_DIFFUSE =>{
            let w : Vector = orienting_normal; 
            let mut u : Vector; let mut v : Vector;
            if (w.x > K_EPS){
                u = vector::normalize(vector::cross(Vector{x:0.0,y:1.0,z:0.0}, w));
            }else{
                u = vector::normalize(vector::cross(Vector{x:1.0,y:0.0,z:0.0}, w));
            }
            v = vector::cross(w, u);

            //const だと通らない -can't capture dynamic environment in fn() item, use clouser instead-
            let r1 : f64 = 2.0 * K_PI * rnd.next01();
            let r2 : f64 = rnd.next01();
            let r2s : f64 = r2.sqrt();

            let dir = vector::normalize(u * r1.cos() * r2s + v * r1.sin() * r2s + w * (1.0-r2).sqrt());

            incoming_radiance = radiance(Ray{org:hitpoint.position,dir:dir}, rnd, depth+1);
            weight = now_object.color / russian_roulette_probability;
        }
        ReflectionType::REFLECTION_TYPE_SPECULAR =>{
            incoming_radiance = radiance(Ray{org:hitpoint.position,dir:ray.dir - hitpoint.normal * 2.0 * vector::dot(hitpoint.normal, ray.dir)}, rnd, depth);
            weight = now_object.color / russian_roulette_probability;
        }
        ReflectionType::REFLECTION_TYPE_REFRACTION => { 
            incoming_radiance = Color{x:0.0,y:0.0,z:0.0};
        }
    }

    now_object.emission + vector::multiply(weight, incoming_radiance)
}