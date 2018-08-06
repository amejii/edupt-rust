use vector::Vector;

pub type Color = Vector;

enum ReflectionType{
    REFLECTION_TYPE_DIFFUSE,        
    REFLECTION_TYPE_SPECULAR,
    REFLECTION_TYPE_REFRACTION,
}

const K_IOR : f64 = 1.5;