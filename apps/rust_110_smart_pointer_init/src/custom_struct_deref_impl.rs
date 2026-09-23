use std::ops::Deref;

pub fn custom_struct_deref_trait_implementation() {
    println!("\n\n----------Custom Struct With Deref Trait Implementation----------------\n\n");

    let x = 7;
    let y = MyBox::new(x);
    assert_eq!(7, *y);
    println!("*y = {}", *y); // *y = 7
}

// tuple struct creation (struct with only value/s, no key/s)
struct MyBox<T>(T);

impl <T> MyBox<T> {
    fn new(x: T) -> Self {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
