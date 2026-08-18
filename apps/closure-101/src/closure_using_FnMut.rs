pub fn call_for_FnMut() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1; // this is valid, as the ownership doesn't change, only being mutated through mutable reference
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");

    // the operation below is not valid
    let mut sort_operations: Vec<String> = vec![];
    let value = String::from("closure called");

    list.sort_by_key(|r| {
        // sort_operations.push(value); // this is not allowed as pushing will move the ownership and the 
        // closure inside of `sort_by_key` implements the trait `FnMut` which doesn't support moving ownership operation in closure body
        r.width // this is fine, as this doesn't move ownership
    });
    println!("{list:#?}");
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
