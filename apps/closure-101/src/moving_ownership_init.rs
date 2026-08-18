use std::thread;

pub fn move_ownership() {
    let v = vec![1, 2, 3];
    println!("Before defining closure: {v:?}");

    thread::spawn(move || println!("From another thread {v:?}"))
    .join()
    .unwrap();

    println!("After calling the closure the v variable is unreachable, as its ownership been moved");
}
