pub fn init_lifetime_for_ref_validaiton() {
    println!("\n\n-------------Use of lifetime for reference validation---------------------\n\n");
    let s = sample_string_return("Hello");
    println!("{s}");
}


// fn longest_str(x: &str, y: &str) -> &str {
    // if x.len() > y.len() {x} else {y}
// }

fn sample_string_return(s1: &str) -> &str {
    "abc"
}

// the code will not compile, as the local variable is cleaned up after the function's scope ends, only variable defined in the parameter will survive and passed through the returned &str
// also note, when we're returning a borrowed reference, its actually the pointer to a memory address, not the actual value
// fn sample_string_return(s1: &str) -> &str {
//     let x = format!("{} {}", "abc", s1);
//     &x
// }
