pub fn slicing_with_slice() {
    println!("\n---------------Hello from slicing_with_slice---------------------\n");
    let words: String = String::from("Hello World!");
    let first_word = &words[0..5];
    let second_word = &words[5..(words.len() - 1)];
    println!("first_word = {first_word} & second_word = {second_word}");

    let full_word = all_word(&words);
    println!("full_word = {full_word}");
}

fn all_word(s: &String) -> &str {
    s
}