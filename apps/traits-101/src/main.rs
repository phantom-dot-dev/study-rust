mod traits_init;
mod trait_second;
mod trait_with_conditional_method_implementation;
mod lifetime_for_ref_validation;

use traits_init::init_trait;
use trait_second::trait_second;
use trait_with_conditional_method_implementation::trait_with_conditional_method_impl;
use lifetime_for_ref_validation::init_lifetime_for_ref_validaiton;

fn main() {
    println!("Hello, world, again again");
    init_trait();
    trait_second();
    trait_with_conditional_method_impl();
    init_lifetime_for_ref_validaiton();
}
