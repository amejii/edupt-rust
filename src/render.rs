//many things are use here.
//organize later...
use radiance::{radiance,Ray,Random,Vector,normalize,cross};
use ppm::{Color,save_ppm_file};


pub fn render(width:u32, height:u32, samples:u32, supersamples:u32) -> i32{
    let camera_position = Vector{x:50.0,y:52.0,z:220.0};
    let camera_dir      = normalize(Vector{x:0.0,y:-0.04,z:-1.0});
    let camera_up       = Vector{x:0.0,y:1.0,z:0.0};

    let screen_width  = 30.0 * width as f64 / height as f64;
    let screen_height = 30.0;

    let screen_dist   = 40.0;
    let screen_x = normalize(cross(camera_dir, camera_up)) * screen_width;
    let screen_y = normalize(cross(screen_x, camera_dir)) * screen_height;
    let screen_center = camera_position + camera_dir * screen_dist;

    //using vector instead of array, because array need to know size at compile time, 
    //therefore it can't use variable as index number
    let mut image : Vec<Color> = vec![Color{x:0.0,y:0.0,z:0.0};(width*height) as usize];
    
    for y in 0..height {
        println!("Rendering(y = {}) {} %",y,100.0*y as f64 / (height as f64 / 1.0));
        let mut rnd = Random::new(y+1);
        for x in 0..width{
            let mut image_index = (height - y - 1) * width + x;
            for sy in 0..supersamples{
                for sx in 0..supersamples{
                    let mut accumulated_radiance = Color{x:0.0,y:0.0,z:0.0};
                    for _s in 0..samples{
                        let rate : f64 = 1.0 / supersamples as f64;
                        let r1 : f64 = sx as f64 * rate + rate / 2.0;
                        let r2 : f64 = sy as f64 * rate + rate / 2.0;

                        let screen_position = 
                            screen_center + screen_x * 
                            ((r1 + x as f64) / width as f64 - 0.5) + 
                            screen_y * ((r2 + y as f64) / height as f64 - 0.5);

                        let dir = normalize(screen_position - camera_position);
                        accumulated_radiance = accumulated_radiance +
                            radiance(Ray{org:camera_position,dir:dir}, &mut rnd, 0) / samples as f64 / (supersamples * supersamples) as f64;
                    }
                    image[image_index as usize] = image[image_index as usize] + accumulated_radiance;
                }
            }
        }
    }

    save_ppm_file("image.ppm", &image, width as i32, height as i32);
    0
}