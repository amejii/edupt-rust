use std::ops::{Add,Sub,Mul,Div};
use std::fmt;
use vecmath::traits::Sqrt;

#[derive(Debug, Copy, Clone)]
//original edupt make "Vec" for class name.
//With this program, use "Vector" for class name instead.
//Because "Vec" is used for vector collection.
pub struct Vector{
    pub x : f64, 
    pub y : f64, 
    pub z : f64,
}

impl Vector {
     pub fn new(x : f64, y : f64, z : f64) -> Vector{
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

//Add, Sub, Mul, Div are operator implementation for Vector class
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

impl Mul<f64> for Vector{
    type Output = Vector;

    fn mul(self, other: f64) -> Vector{
        Vector{x: self.x*other, y: self.y*other, z: self.z*other}
    }
}

impl Div<f64> for Vector{
    type Output = Vector;

    fn div(self, other: f64) -> Vector{
        if other == 0.0{
            panic!("Cannot divide by zero.");
        }
        Vector{x: self.x/other, y: self.y/other, z: self.z/other}
    }
}

impl fmt::Display for Vector{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "({}, {}, {})",self.x, self.y, self.z)
    }
}

impl Mul<Vector> for f64{
    type Output = Vector;

    fn mul(self, b: Vector) -> Vector{
        b * self
    }
}

pub fn normalize(v : Vector) -> Vector{
    v * (1.0 / v.length())
}

pub fn multiply(v1: Vector, v2: Vector) -> Vector{
    Vector{x: v1.x * v2.x, y: v1.y * v2.y, z: v1.z*v2.z}
}

pub fn dot(v1: Vector, v2: Vector) -> f64{
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn cross(v1: Vector, v2: Vector) -> Vector{
    Vector{
        x: (v1.y * v2.z) - (v1.z * v2.y),
        y: (v1.z * v2.x) - (v1.x * v2.z),
        z: (v1.x * v2.y) - (v1.y * v2.x)
    }
}