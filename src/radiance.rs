//many things are use here.
//organize later...
use constant::{K_INF,K_EPS,K_PI};
use sphere::Sphere;
use intersection::{Hitpoint,Intersection};
use vector;
use vector::Vector;
use ray::Ray;
use material::{Color,ReflectionType,K_IOR};
use vecmath::traits::Sqrt;
use random::Random;
use scene::{intersect_scene,spheres};
use std;

const K_BACKGROUND_COLOR : Color = Color{x:0.0,y:0.0,z:0.0};
const K_DEPTH : i32 = 5;
const K_DEPTH_LIMIT : i32 = 64;

pub fn radiance(ray: Ray, mut rnd: &mut Random, depth: i32) -> Color{
    let mut intersection = Intersection::new();
    if(!intersect_scene(&ray, &mut intersection)){
        return K_BACKGROUND_COLOR;
    }

    
    let now_object : &Sphere = &spheres[intersection.object_id as usize]; //array's index number need to be usize type
    let hitpoint : Hitpoint = intersection.hitpoint;
    //can't write as xxx ? true : false, instead if statement work as expression in rust (therefore it return a value)
    let orienting_normal : Vector = if (vector::dot(hitpoint.normal, ray.dir) < 0.0){
        hitpoint.normal 
    }else{
        (-1.0 * hitpoint.normal)
    };
    let mut russian_roulette_probability = now_object.color.x.max(now_object.color.y.max(now_object.color.z));
    
    if(depth > K_DEPTH_LIMIT){
        russian_roulette_probability *= (0.5_f64).powf((depth-K_DEPTH_LIMIT).into());
    }

    if (depth > K_DEPTH){
        if(rnd.next01() >= russian_roulette_probability){
            return now_object.emission;
        }
    }else{
        russian_roulette_probability = 1.0;
    }
    
    let incoming_radiance : Color;
    let mut weight : Color = Color{x:1.0, y:1.0, z:1.0};
    //this match doesn't need to loop.
    //But it include break point in match block and, break is only work for loop block.
    //So cover match block with loop block.
    loop{match(now_object.reflection_type){
        ReflectionType::REFLECTION_TYPE_DIFFUSE =>{
            let w : Vector = orienting_normal; 
            let mut u : Vector; let mut v : Vector;
            if (w.x > K_EPS){
                u = vector::normalize(vector::cross(Vector{x:0.0,y:1.0,z:0.0}, w));
            }else{
                u = vector::normalize(vector::cross(Vector{x:1.0,y:0.0,z:0.0}, w));
            }
            v = vector::cross(w, u);

            //if define with const, it sayw error (-can't capture dynamic environment in fn() item, use clouser instead-)
            //clouser??? sorry, ignore for now. may be TBD.
            let r1 : f64 = 2.0 * K_PI * rnd.next01();
            let r2 : f64 = rnd.next01();
            let r2s : f64 = r2.sqrt();
            
            let dir = vector::normalize(u * r1.cos() * r2s + v * r1.sin() * r2s + w * (1.0-r2).sqrt());

            incoming_radiance = radiance(Ray{org:hitpoint.position,dir:dir}, rnd, depth+1);
            weight = now_object.color / russian_roulette_probability;
        }
        ReflectionType::REFLECTION_TYPE_SPECULAR =>{
            incoming_radiance = radiance(Ray{org:hitpoint.position,dir:ray.dir - hitpoint.normal * 2.0 * vector::dot(hitpoint.normal, ray.dir)}, rnd, depth+1);
            weight = now_object.color / russian_roulette_probability;
        }
        ReflectionType::REFLECTION_TYPE_REFRACTION => { 
            let reflection_ray = Ray{org:hitpoint.position,dir:ray.dir - hitpoint.normal * 2.0 * vector::dot(hitpoint.normal, ray.dir)};
            let into : bool = vector::dot(hitpoint.normal, orienting_normal) > 0.0;

            let nc = 1.0;
            let nt = K_IOR;
            let nnt = if (into) {nc / nt} else {nt / nc};
            let ddn = vector::dot(ray.dir, orienting_normal);
            let cos2t = 1.0 - nnt * nnt * (1.0 - ddn * ddn);

            if (cos2t < 0.0){
                incoming_radiance = radiance(reflection_ray, rnd, depth+1);
                weight = now_object.color / russian_roulette_probability;
                break;  //break early
            } 

            let reflection_ray = Ray{org:hitpoint.position, 
                dir: vector::normalize(ray.dir * nnt - hitpoint.normal * if(into){1.0}else{-1.0} * (ddn * nnt + cos2t.sqrt()))};

            let a = nt - nc;
            let b = nt + nc;
            let R0 = (a * a) / (b * b);

            let c = 1.0 - if(into){-ddn}else{
                vector::dot(reflection_ray.dir, -1.0 * orienting_normal)
            };
            let Re = R0 + (1.0 - R0) * c.powf(5.0);
            let nnt2 = if(into){(nc/nt).powf(2.0)}else{(nt/nc).powf(2.0)};
            let Tr = (1.0 - Re) * nnt2;

            let probability = 0.25 + 0.5 * Re;
            if(depth > 2){
                if(rnd.next01() < probability){
                    incoming_radiance = radiance(reflection_ray, rnd, depth+1) * Re;
                    weight = now_object.color / (probability * russian_roulette_probability);
                }else{
                    incoming_radiance = radiance(reflection_ray, rnd, depth+1) * Tr;
                    weight = now_object.color / ((1.0 - probability) * russian_roulette_probability);
                }
            }else{
                //this statement says error, that reflection_ray and rnd moved with first radiance() call.
                //So the second radiance() call is not valid. To avoid this implement Copy trait for these structs.
                incoming_radiance = radiance(reflection_ray, rnd, depth+1) * Re + radiance(reflection_ray, rnd, depth+1) * Tr;
                weight = now_object.color / (russian_roulette_probability);
            }
        }
    } break; }// always break at the end of loop

    now_object.emission + vector::multiply(weight, incoming_radiance)
}