use std::fmt::Display;

pub fn trait_with_conditional_method_impl() {
    println!("\n\n------------Trait with conditional method implementations----------------\n\n");

    let pair_same = Pair::new(12, 12);
    pair_same.compare();

    let pair_different = Pair::new(7, 6);
    pair_different.compare();
}


struct Pair<T> {
    x: T,
    y: T,   
}

impl <T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl <T: Display + PartialOrd> Pair<T> {
    fn compare(&self) {
        if self.x == self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

