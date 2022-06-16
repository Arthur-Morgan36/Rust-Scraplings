fn generate_fibonacci_sequence(number_of_terms: usize) -> Vec<u16> {
    if number_of_terms == 0 { return vec![] };
    if number_of_terms == 1 { return vec![0] };

    let mut initial_vec: Vec<u16> = vec![0, 1];

    for i in 0..(number_of_terms - initial_vec.len()) {
        initial_vec.push(initial_vec[i] + initial_vec[i + 1]);
    }

    return initial_vec;
}

fn main() {
    let ten_first_items: Vec<u16> = generate_fibonacci_sequence(10);
    println!("Sequence: {:?}", ten_first_items);
}