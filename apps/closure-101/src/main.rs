mod moving_ownership_init;
mod closure_using_FnMut;

use moving_ownership_init::move_ownership;
use closure_using_FnMut::call_for_FnMut;

fn main() {
    println!("Moving Ownership Example---------------------- \n");
    move_ownership();

    println!("\n\n--------------------------Closure example using FnMut trait-------------------------------\n\n");
    call_for_FnMut();
}
