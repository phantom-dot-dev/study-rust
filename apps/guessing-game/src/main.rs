// #![allow(unused)]


fn main() {
    let num_array = [1, 2, 3, 4, 5, 6, 7];

    for element in num_array {
        println!("printing {element}");
    }

    // with both index and element
    for (index, element) in num_array.iter().enumerate() {
        println!("num_array index: {index} and element: {element}")
    }

    // for with reverse iteration
    println!("Printing reverse order");
    for element in num_array.iter().rev() {
        println!("printing {element}");
    }

    // `for` to iterate over range in reverse order
    for element in (1..=7).rev() {
        println!("printing {element}");
    }
}
