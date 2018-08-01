extern crate edupt;
//const kPI : f64 = 3.14159265358979323846;
//const kINF : f64 = 1e128;
//const kEPS : f64 = 1e-6;

use edupt::vector::Vector;

struct Hitpoint{
    distance : f64,
}

fn main() {
    let v1 = Vector{x:1.0,y:2.0,z:3.0};
    let v2 = Vector{x:1.0,y:2.0,z:3.0};
    
    println!("{}\n", v1.length());
    println!("{}\n", v2.length());
    println!("{}\n", v1 * v2);
    println!("{}\n", v1 / v2);
}
