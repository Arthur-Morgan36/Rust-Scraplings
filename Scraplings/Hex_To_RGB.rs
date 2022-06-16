use std::u8;

#[allow(dead_code)]
#[derive(Debug)]
struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

fn hex_to_rgb(hex: String) -> RGB {
    let len_hex = hex.replace("#", "");
    let working_hex: String = if len_hex.len() < 6 {
        "0".repeat(6 - &len_hex.len()) + &len_hex
    } else {
        len_hex
    };

    let rgb: Vec<u8> = vec![&working_hex[0..2], &working_hex[2..4], &working_hex[4..6]]
        .into_iter()
        .map(|clr| u8::from_str_radix(clr, 16).expect("Invalid character(s) found in hex code!"))
        .collect();

    return RGB {
        red: rgb[0],
        green: rgb[1],
        blue: rgb[2],
    };
}

fn main() {
    let hex: String = String::from("#2F3136");
    println!("{:#?}", hex_to_rgb(hex));
}
