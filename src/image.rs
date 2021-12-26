use std::io;     
use std::io::prelude::*; 

pub fn ppm_image() {
    const WIDTH:i16 = 256;
    const HEIGHT:i16 = 256;

    println!("P3\n{} {}\n255", WIDTH, HEIGHT);

    for j in (0..(HEIGHT)).rev() {
        println!("\rScanlines remaining: {} ", j);
        io::stdout().flush().ok().expect("Could not flush stdout");
        for i in 0..WIDTH {
            let r:f64 = i as f64 / (WIDTH - 1) as f64;
            let g:f64 = j as f64 / (HEIGHT - 1) as f64;
            let b:f64 = 0.25;

            let ir: i16 = (255.999 * r) as i16;
            let ig: i16 = (255.999 * g) as i16;
            let ib: i16 = (255.999 * b) as i16;
            println!("{} {} {}", ir, ig, ib);
        }
    }
    println!("\nDone");
}