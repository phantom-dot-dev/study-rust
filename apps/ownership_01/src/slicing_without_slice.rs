pub fn slicing_without_slice() {
    println!("\n---------------no_slice_find_word------------------\n");
    let letter = String::from("Hello World Again");
    let i = first_word(&letter);

    println!("first space of sentence \"{letter}\" is at index = {i}");

    println!("\n----------Slicing words manual-------------\n");
    let word_col = slicing_word_manual(&letter);
    println!("words = {:?}", word_col);

}

// In idiomatic Rust, functions do not take ownership of their arguments unless they need to, and the reasons for that will become clear as we keep going
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    println!("bytes = {:?}", bytes); // [72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100] for "Hello World"
    // printing something with `{:?}` (`debug format specifier`) is for debugging

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // here `b' '` is a byte literal `b` for a single space
            return index;
        }
    }

    s.len()
}

fn slicing_word_manual(words: &String) -> Vec<String> {
    // variable vector (container) with all the words
    let mut word_vector: Vec<String> = vec![];
    let mut a_word = String::from("");
    let bytes = words.as_bytes();

    // current pointer is the loop
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' || index == bytes.len() - 1 {
            if index == bytes.len() -1 {
                a_word.push(*&item as char);
            }
            word_vector.push(a_word.clone());
            a_word.clear();
            continue;
        }
        a_word.push(*&item as char); // `a_word.push(item as char);` will also work, plain and simple 
    }
    return word_vector;
}
