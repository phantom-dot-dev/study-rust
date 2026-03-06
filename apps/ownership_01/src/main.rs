#![allow(unused)]
mod ownership_and_function;
mod return_value_and_variable_scope;
mod reference_and_borrowing;
mod borrowing_and_mutation;
mod slicing_without_slice;
mod slicing_with_slice;

mod another_file;

// use std::io;
use ownership_and_function::ownership_and_function;
use return_value_and_variable_scope::return_value_and_variable_scope;
use reference_and_borrowing::reference_and_borrowing;
use borrowing_and_mutation::borrowing_and_mutation;
use slicing_without_slice::slicing_without_slice;
use slicing_with_slice::slicing_with_slice;

use another_file::another_function;

fn main() {
    ownership_and_function();
    return_value_and_variable_scope();
    reference_and_borrowing();
    borrowing_and_mutation();
    slicing_without_slice();
    slicing_with_slice();

    another_function(); // testing multi-file projects
}
