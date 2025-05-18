fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("Panjang dari '{s1}' adalah {len}");

    let mut s = String::from("Ackxle");
    change(&mut s);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", Lexiria");
    println!("{some_string}");
}
