/*
    This function does the same thing as Vec.iter().max() except that it can be used with floating point values.
    Originally used that as a way to get the biggest element of a vector because I didn't know any better but this turned out to be a great creation.

    * The function assumes the vector has no NaN values; if you'd like to adda a check for that you're welcome to (just a simple guard clause if)
    Used a generic in case I have to use it with multiple types.
*/

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

fn main() {
    let vector: Vec<i8> = vec![0, -8, -22, 127, 96];
    let vector_f: Vec<f32> = vec![1.22, -8.855, 45896.0];
    println!(
        "Max U8 Vector: {}\nMax F32 Vector: {}\nMin U8 Vector: {}\nMin F32 Vector: {}",
        max(&vector),
        max(&vector_f),
        min(&vector),
        min(&vector_f)
    );
}
