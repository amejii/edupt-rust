use material::Color;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::path::Path;

fn clamp(x:f64) -> f64{
    if x < 0.0{
        return 0.0;
    }        
    if x > 1.0{
        return 1.0;
    }

    x
}

fn to_int (x: f64) -> i32{
    //because rust does not allow to calcutaion between i32 and f64,
    //make all constant value as floating point.
    (clamp(x).powf(1.0/2.2) * 255.0 + 0.5) as i32
}

pub fn save_ppm_file(filename : &str, image : &[Color], width : i32, height : i32){
    let path = format!("{}{}","out/".to_string(),filename);
    let path = Path::new(&path);

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why.description()),
        Ok(file) => file,
    };
    
    let header = format!("P3\n{} {}\n{}\n",width.to_string(),height.to_string(),255.to_string());
    match file.write_all(header.as_bytes()){
        Err(why) => {
            panic!("couldn't write to {}: {}", path.display(),
                                               why.description())
        },
        Ok(_) => println!("successfully wrote to {}", path.display()),
    }
    
    for element in image.iter(){
        let data = format!("{} {} {} ",to_int(element.x),to_int(element.y),to_int(element.z));
        file.write_all(data.as_bytes());
    }
}