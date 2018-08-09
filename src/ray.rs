use vector::Vector;

pub struct Ray{
    pub org : Vector,
    pub dir : Vector,
}

impl Ray {
    fn new(org : Vector, dir : Vector) -> Ray{
        Ray{
            org : org,
            dir : dir,
        }
    }
}