pub fn box_smart_pointer_init() {
    println!("\n----------------- Smart Pointer Init 101 --------------------\n");
    simple_box();
}


fn simple_box() {
    let x = Box::new(7);
    println!("x = {x}");
}


// const list (recursive pattern), ie, (1, (2, (3, Nil)))
enum List {
    // Cons(i32, List) // because we included another List type inside of the List type, it's a recursive type, without know size, rust will flag this as an error
    Cons(i32, Box<List>), // with the Box<T>, we've created a smart pointer, Now rust compiler knows, it only have to store the pointer address (fixed size for everything) to the stack memory and the actual data will be stored in the heap
    Nil,
}

use crate::box_smart_pointer::List::{Cons, Nil}; // without this shortcut creation, we'll have to call each of the Cons and Nil by List::Cons and list::Nil

// Calling the Cons list with different recursion levels
fn call_cons_list() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let list3 = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    let list2 = Cons(1, Box::new(Nil));
    let List4 = Nil;
    let list5 = List::Nil; // without using the short-cut created by `use` statement
}
