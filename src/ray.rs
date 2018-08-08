use vector::Vector;

struct Ray{
    org : Vector,
    dir : Vector,
}

impl Ray {
    fn new(org : Vector, dir : Vector) -> Ray{
        Ray{
            org : org,
            dir : dir,
        }
    }
}