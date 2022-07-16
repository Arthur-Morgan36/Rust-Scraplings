use std::io::{stdin, stdout, Write};

fn read(input: &mut String) -> () {
  stdout().flush().expect("Failed to flush.");
  stdin().read_line(input).expect("Failed to read input.");
}

struct Calc;

impl Calc {
  fn sum<T: std::ops::Add<Output = T> + Copy>(num1: T, num2: T) -> T {
    num1 + num2
  }
  fn sub<T: std::ops::Sub<Output = T> + Copy>(num1: T, num2: T) -> T {
    num1 - num2
  }
  fn mult<T: std::ops::Mul<Output = T> + Copy>(num1: T, num2: T) -> T {
    num1 * num2
  }
  fn div<T: std::ops::Div<Output = T> + Copy>(num1: T, num2: T) -> T {
    num1 / num2
  }
}

fn main() -> () {
  println!("Welcome to the Rust 🦀 Calculator!\n");

  loop {
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut operator = String::new();

    print!("First number: ");
    read(&mut num1);

    let num1: f32 = match num1.trim().parse::<f32>() {
      Ok(num) => num,
      Err(parse_error) => {
        println!(
          "Only numbers are valid, encountered error: {}.\n",
          parse_error
        );
        continue;
      }
    };

    print!("Second number: ");
    read(&mut num2);
    // todo: Make it so it doesn't ask for num1 again if num2 is invalid (basically the use of continue is not optimal here)
    let num2: f32 = match num2.trim().parse::<f32>() {
      Ok(num) => num,
      Err(_) => {
        println!("Invalid float literal, make sure you entered a valid number.\n");
        continue;
      }
    };

    print!("Operation you'd like to perform: ");
    read(&mut operator);

    let operator: &str = operator.trim();

    let result = match operator {
      "+" => Calc::sum(num1, num2).to_string(),
      "-" => Calc::sub(num1, num2).to_string(),
      "*" => Calc::mult(num1, num2).to_string(),
      "/" => Calc::div(num1, num2).to_string(),
      &_ => {
        // todo: Same as other todo for this purpose, no need to ask for everything again.
        println!("Unknown operator; only valid options are '+', '-', '*', '/'.\n");
        continue;
      }
    };

    println!(
      "The result of {} {} {} is {}.\n",
      num1, operator, num2, result
    );
  }
}
