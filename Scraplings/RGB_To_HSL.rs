#[allow(dead_code)]
#[derive(Debug)]
struct HSL {
    hue: u8,
    saturation: u8,
    luminosity: u8,
}

fn max<T: std::cmp::PartialOrd + Copy>(vec: &Vec<T>) -> &T {
    let mut max: &T = &vec[0];
    for item in vec {
        if item > max {
            max = item;
        }
    }
    return max;
}

fn min<T: std::cmp::PartialOrd + Copy>(vec: &Vec<T>) -> &T {
    let mut min: &T = &vec[0];
    for item in vec {
        if item < min {
            min = item;
        }
    }
    return min;
}

fn rgb_to_hsl(rgb: Vec<u8>) -> HSL {
    let div_rgb: Vec<f32> = rgb.iter().map(|x| *x as f32 / 255_f32).collect();
    let max_min: [&f32; 2] = [max(&div_rgb), min(&div_rgb)];
    let chroma: f32 = max_min[0] - max_min[1];
    let luminosity: f32 = ((max_min[0] + max_min[1]) * 50_f32).round();

    if max_min[0] == max_min[1] {
        return HSL {
            hue: 0,
            saturation: 0,
            luminosity: luminosity as u8,
        };
    }

    let saturation = if luminosity <= 50_f32 {
        ((chroma / (max_min[0] + max_min[1])) * 100_f32).round()
    } else {
        ((chroma / (2_f32 - max_min[0] - max_min[1])) * 100_f32).round()
    };

    let mut hue: f32 = match max_min[0] {
        red if red == &div_rgb[0] => ((div_rgb[1] - div_rgb[2]) / chroma) + 0_f32,
        green if green == &div_rgb[1] => ((div_rgb[2] - div_rgb[0]) / chroma) + 2_f32,
        blue if blue == &div_rgb[2] => ((div_rgb[0] - div_rgb[1]) / chroma) + 4_f32,
        &_ => 0_f32,
    };

    hue = if hue < 0_f32 {
        (hue * 60_f32).round() + 360_f32
    } else {
        hue * 60_f32
    };

    return HSL {
        hue: hue as u8,
        saturation: saturation as u8,
        luminosity: luminosity as u8,
    };
}

fn main() {
    let rgb: Vec<u8> = vec![250, 243, 226];
    println!("{:?}", rgb_to_hsl(rgb));
}
