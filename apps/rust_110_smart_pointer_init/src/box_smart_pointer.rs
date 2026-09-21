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
    Cons(i32, Box<List>), // because we included another List type inside of the List type, it's a recursive type, without know size, rust will flag this as an error
    Nil
}

use crate::box_smart_pointer::List::{Cons, Nil}; // without this shortcut creation, we'll have to call each of the Cons and Nil by List::Cons and list::Nil

fn call_cons_list() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
