pub fn no_slice_find_word() {
    println!("\n---------------no_slice_find_word------------------\n");
    let letter = String::from("Hello World");
    let i = first_word(&letter);

    println!("index of letter \"{letter}\" is i = {i}");
}


// In idiomatic Rust, functions do not take ownership of their arguments unless they need to, and the reasons for that will become clear as we keep going
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    println!("bytes = {:?}", bytes); // [72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100] for "Hello World"

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' '  { // here `b' '` is a byte literal `b` for a single space
            return index;
        }
    }

    s.len()
}
