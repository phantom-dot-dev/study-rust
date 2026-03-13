pub fn slicing_with_slice() {
    println!("\n---------------Hello from slicing_with_slice---------------------\n");
    let words: String = String::from("Hello World!");
    let first_word = &words[0..5];
    let second_word = &words[5..(words.len() - 1)];
    println!("first_word = {first_word} & second_word = {second_word}");

    let full_word = all_word(&words);
    println!("full_word = {full_word}");

    println!("\n------------exploring tuple---------------\n");
    exploring_tuple_ref();
}

fn all_word(s: &String) -> &str {
    s
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            
        }
    }
    
    s
}


fn exploring_tuple_ref() {
    let x = 10;
    let y = 20;
    let z = 30;

    let tup = (x, &y, &z);
    let (xd, &yd, zd) = tup;
    println!("destructed xd = {xd} and yd = {yd} and zd = {zd}");
    /**
     * note: while destructuring from tuple, prefixing with `&` will 
     */

    let num_array = [1, 2, 3, 4, 5, 6, 7];

    for element in num_array {
        println!("printing {element}");
    }

    // with both index and element
    for (index, &element) in num_array.iter().enumerate() {
        println!("num_array index: {index} and element: {element}")
    }
}