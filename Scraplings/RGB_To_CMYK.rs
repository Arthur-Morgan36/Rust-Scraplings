#[derive(Debug)]
#[allow(dead_code)]
struct CMYK {
    c: u8,
    m: u8,
    y: u8,
    k: u8,
}

fn max<T: std::cmp::PartialOrd + Copy>(vec: &Vec<T>) -> &T {
    let mut max: &T = &vec[0];
    for i in 1..vec.len() {
        if i > 0 {
            if vec[i] > vec[i - 1] {
                max = &vec[i]
            } else {
                max = &vec[i - 1]
            }
        }
    }
    return max;
}

fn rgb_to_cmyk(rgb: Vec<u8>) -> CMYK {
    if rgb.iter().all(|color| color == &0) {
        return CMYK {
            c: 0,
            m: 0,
            y: 0,
            k: 100,
        };
    }

    let div_rgb: Vec<f32> = rgb.into_iter().map(|x| x as f32 / 255 as f32).collect();
    let key: f32 = 1_f32 - max(&div_rgb);

    let calc = |val: &f32| (1_f32 - val - &key) / (1_f32 - &key);

    return CMYK {
        c: (calc(&div_rgb[0]) * 100_f32).round() as u8,
        m: (calc(&div_rgb[1]) * 100_f32).round() as u8,
        y: (calc(&div_rgb[2]) * 100_f32).round() as u8,
        k: (key * 100_f32).round() as u8,
    };
}

fn main() {
    let rgb: Vec<u8> = vec![25, 65, 155];
    println!("{:#?}", rgb_to_cmyk(rgb));
}
