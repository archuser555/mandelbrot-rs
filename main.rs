use std::fs::File;
use std::io::Write;

fn mandelfind(cr: f32, ci: f32, max_i: i32) -> i32 {
    let mut i: i32 = 0;
    let mut zr: f32 = 0.0;
    let mut zi: f32 = 0.0;
    while i < max_i && zr * zr + zi * zi < 4.0 {
        let temp: f32 = zr * zr - zi * zi + cr;
        zi = 2.0 * zr * zi + ci;
        zr = temp;
        i += 1;
    }
    return i;
}

fn map2real(x: i32, width: i32, minr: f32, maxr: f32) -> f32 {
    return x as f32 * ((maxr - minr) / width as f32) + minr;
}

fn map2imaginery(y: i32, height: i32, mini: f32, maxi: f32) -> f32 {
    return y as f32 * ((maxi - mini) / height as f32) + mini;
}

fn main() {
    //Let's Declare Some Variable's
    let maxn: i32;
    let (minr, maxr, mini, maxi) : (f32, f32, f32, f32);
    maxn = 255; minr = -1.5; maxr = 0.7; mini = -1.0; maxi = 1.0;
    let mut result = File::create("./result/mb.ppm").expect("Problem 1");

    write!(result, "P3\n1000 1000\n256\n"); //Give Basic PPM Data

    for x in 0..1000 { //Row's?
        for y in 0..1000 { //Pixel's In Row?
            let cr: f32 = map2real(x, 1000, minr, maxr);
            let ci: f32 = map2imaginery(y, 1000, mini, maxi);
            let n: i32 = mandelfind(cr, ci, maxn);
            let r: i32 = n % 256;
            let g: i32 = n % 256;
            let b: i32 = n % 256;
            write!(result,"{r} {g} {b} ");
        }
    }
}