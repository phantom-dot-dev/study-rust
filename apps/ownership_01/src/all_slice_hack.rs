pub fn all_slice_hacks() {
    println!("\n----------------All Slice Hacks------------------\n");
    slice_type_examples();
}

fn slice_type_examples() {
    let s = String::from("Hello world!");
    let mut temp: &mut &str = &mut "Again";

    let word_1 = &s[0..5];
    let word_2 = &s[6..12];
    println!("word_1 = {word_1} and word_2 = {word_2}");

    let all_words = &s[..]; // without specifying starting and ending range will slice the full sequence
    let hello = &s[..5]; // starting is not specified
    let world = &s[6..]; // ending is not specified
    println!("hello = {hello} and world = {world}");

}