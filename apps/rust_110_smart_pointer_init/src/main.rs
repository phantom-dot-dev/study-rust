mod box_smart_pointer;
mod custom_struct_deref_impl;

use box_smart_pointer::box_smart_pointer_init;
use custom_struct_deref_impl::custom_struct_deref_trait_implementation;

fn main() {
    box_smart_pointer_init();
    custom_struct_deref_trait_implementation();
}
