use std::env;
use std::string::String;
pub fn create() {
    let imgx = 800;
    let imgy = 800;

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    for x in 0..imgx { println!("x is {} out of {}", x, imgx);
        for y in 0..imgy {
            let cx = y as f32 * scalex - 1.0;
            let cy = x as f32 * scaley - 1.5;

            let c = num_complex::Complex::new(-0.4, 0.6);
            let mut z = num_complex::Complex::new(cx, cy);
            let mut i = 0;
            while i < 255 && z.norm() < 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);
            let data = (*pixel as image::Rgb<u8>).data;
            *pixel = image::Rgb([data[0], i as u8, data[2]]);
        }
    }

    let filename = get_filename();
    imgbuf.save(filename).unwrap();
}

pub fn get_filename() -> String {
    let mut filename = String::from("fractal.png");
    let args: Vec<String> = env::args().collect();

    let output_index: Option<usize> = get_output_index(&args);

    match output_index {
        Some(usize) => {
            filename = args[output_index.unwrap() + 1].clone();
        }
        None => println!("No output flag (-o), default name used (fractal.png)"),
    }

    filename
}

fn get_output_index(args: &Vec<String>) -> Option<usize> {
    let mut correct_index: Option<usize> = None;
    for (i, item) in args.iter().enumerate() {
        if item == "-o" {
            correct_index = Some(i);
        }
    }
    correct_index
}
