/*
    Can't be bothered to continue coding this
    this code has errors btw
*/

struct HSL {
    Hue: u8,
    Saturation: u8,
    Luminosity: u8,
}

fn RGBToHSL(RGB: Vec<f32>) -> HSL {
    let divRGB: Vec<f32> = RGB.iter().map(|x| x / 255f32).collect();
    let max_min: [&f32; 2] = [&divRGB.iter().min().unwrap(), &divRGB.iter().max().unwrap()];
}

fn main() {
    println!("Rust > C++");
}
