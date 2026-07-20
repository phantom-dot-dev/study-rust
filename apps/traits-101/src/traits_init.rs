pub fn init_trait() {
    println!("\n\n-----------------Traits init---------------\n\n");
    
    let news = News { title: String::from("Hello world!"), story: String::from("Again Hello World!") };
    notify(&news);
    notify_second(&news);
}


// trait as parameter
pub trait Summary {
    fn summarize(&self) -> String;
}

// here the `item` parameter accept any type that implement `Summary` trait, and all functionality become available of that trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// trait with bound (syntax)
pub fn notify_second<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
    // the code above, T is bound to Summary trait
}


struct News {
    title: String,
    story: String,
}


impl Summary for News {
    fn summarize(&self) -> String {
        format!("Read More: {}", self.title)
    }
}
