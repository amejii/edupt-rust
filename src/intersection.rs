use vector::Vector;
use constant::K_INF;

#[derive(Debug)]
struct Hitpoint {
    distance: f64,
    normal: Vector,
    position: Vector,
}

impl Hitpoint{
    fn new () -> Hitpoint{
        Hitpoint{
            distance : K_INF,
            normal : Vector{x:0.0, y:0.0, z:0.0},
            position : Vector{x:0.0, y:0.0, z:0.0},
        }
    }
}

#[derive(Debug)]
struct Intersection {
    hitpoint: Hitpoint,
    object_id : i32,   
}

impl Intersection {
     fn new () -> Intersection{
         Intersection{
             object_id : -1,
             hitpoint : Hitpoint::new(),
         }
     }
}