use vector::Vector;
pub use constant::K_INF;

#[derive(Debug)]
pub struct Hitpoint {
    pub distance: f64,
    pub normal: Vector,
    pub position: Vector,
}

impl Hitpoint{
    //rust don't have constructor... use new function like constructor.
    //everywhere where constructor is defined in original edupt code,
    //new function will be implemented instead in edupt-rust.
    pub fn new () -> Hitpoint{
        //this is actual constructor in rust
        Hitpoint{
            distance : K_INF,
            normal : Vector{x:0.0, y:0.0, z:0.0},
            position : Vector{x:0.0, y:0.0, z:0.0},
        }
    }
}

#[derive(Debug)]
pub struct Intersection {
    pub hitpoint: Hitpoint,
    pub object_id : i32,   
}

impl Intersection {
     pub fn new () -> Intersection{
         Intersection{
             object_id : -1,
             hitpoint : Hitpoint::new(),
         }
     }
}