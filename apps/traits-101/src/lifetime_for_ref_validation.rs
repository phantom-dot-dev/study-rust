pub fn init_lifetime_for_ref_validaiton() {
    println!("\n\n-------------Use of lifetime for reference validation---------------------\n\n");
    let s = sample_string_return("Hello");
    println!("{s}");

    let a: i32;
    {
        let b = 7;
        a = b;
    }
}

// the function below will not compile, as rust compiler need to know which borrowed parameter it need to retain for longer time, as 
// when passing a single borrowed parameter, rust compiler knows the lifetime for the parameter, hence will not destroy its reference, but
// when passing more than one parameter, rust compiler needs to know which borrowed parameter it needs to retain, at this point we need to specify lifetime explicitly
// the rust compiler does so to prevent from creating dangling pointer, so that no pointer (borrowed-reference) can be pointed to a deleted value in the memory 
// fn longest_str(x: &str, y: &str) -> &str {
    // if x.len() > y.len() {x} else {y}
// }

fn sample_string_return(s1: &str) -> &str {
    // "abc" // works. as string literal has static lifetime, it stays in the memory as long as the program is running ("abc" is embedded into the binary)
    // when the program runs, it's binary is loaded into ram memory from hard disk,  and the returned address `&str` pointer will be valid until the variable holding that goes out of scope
    let sth = "abc"; // because string literal has static lifetime, the value "abc" will never be cleaned up, the `sth` variable/reference will be cleaned up when function's scoped ended,
    // but the returned address captured from another variable outside of this function will still point to a valid memory address, hence it will work  
    sth
}

// also note, when we're returning a borrowed reference, its actually the pointer to a memory address, not the actual value
// the code will not compile, as the local variable is cleaned up after the function's scope ends, only variable defined in the parameter will survive and passed through the returned &str
// when passing a single borrowed parameter, rust compiler knows the lifetime for the parameter, hence will not destroy its reference
// fn sample_string_return(s1: &str) -> &str {
//     let x = format!("{} {}", "abc", s1); // it will be deleted, as it's not a string literal with static lifetime
//     &x // so when the function ends, the value of x will be deleted, and &x becomes a dangling pointer, which rust compiler will not allow
// }
