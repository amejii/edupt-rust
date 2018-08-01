use std::ops::{Add,Sub,Mul,Div};
use std::fmt;
use vecmath::traits::Sqrt;

#[derive(Debug, Copy, Clone)]
pub struct Vector{
    pub x : f64, 
    pub y : f64, 
    pub z : f64,
}

impl Vector {
     fn new(x : f64, y : f64, z : f64) -> Vector{
         Vector{
             x : x,
             y : y,
             z : z,
         }
     }

     pub fn length_squared(self) -> f64 {
         self.x * self.x + self.y * self.y + self.z * self.z
     }

     pub fn length(self) -> f64{
         Sqrt::sqrt(self.length_squared())
     }
}



impl Add for Vector{
    type Output = Vector;

    fn add(self, other: Vector) -> Vector{
        Vector{x: self.x+other.x, y: self.y+other.y, z:self.z+other.z}
    }
}

impl Sub for Vector{
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector{
        Vector{x: self.x-other.x, y: self.y-other.y, z: self.z-other.z}
    }
}

impl Mul for Vector{
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector{
        Vector{x: self.x*other.x, y: self.y*other.y, z: self.z*other.z}
    }
}

impl Div for Vector{
    type Output = Vector;

    fn div(self, other: Vector) -> Vector{
        if other.x == 0.0 || other.y == 0.0 || other.z == 0.0{
            panic!("Cannot divide by zero.");
        }
        Vector{x: self.x/other.x, y: self.y/other.y, z: self.z/other.z}
    }
}

impl fmt::Display for Vector{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "({}, {}, {})",self.x, self.y, self.z)
    }
}
