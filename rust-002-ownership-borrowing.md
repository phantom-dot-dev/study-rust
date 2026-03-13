### Ownership and Borrow:

Docs : https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html and follow part 2 and 3 also

Ownership is the rust's most unique feature, it enables Rust to make memory safety guarantees without needing a garbage collector. Ownership model work together with `borrowing, slices and how rust lays data out in memory`.

Stack vs Heap: 

* All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead.

Stack's data is organized, maintain the `LAFO` pattern, last-in-first-out. 

The heap is less organized. When you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. This process is called allocating on the heap and is sometimes abbreviated as just allocating (pushing values onto the stack is not considered allocating). Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer.


When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.


Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so that you don’t run out of space are all problems that ownership addresses. Once you understand ownership, you won’t need to think about the stack and the heap very often. But knowing that the main purpose of ownership is to manage heap data can help explain why it works the way it does. 


Ownership Rules:

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

```rust
fn main() {                      
    // s is not valid here, since it's not yet declared
    let s = "hello";   // s is valid from this point forward
    let t = "world";   // t is valid from here
}                      // this scope is now over, and s and t are no longer valid, t removed first, then the s had been removed. As of `stack`, last-in-first-out.
```

### String vs String literal:

* Learn difference between Rust core vs std library first.

Overall `String` type can refer to either core language's string slice `&str` borrowed type or the std library's `String` (owned) type.

Rust core library comes with only one string type, string slice `str` that is usually seen in its borrowed form, `&str`. This string slices are references to some UTF-8 encoded string data stored elsewhere. String literals, for example, are stored in the program’s binary and are therefore string slices.

The std library provide the `String` type (different from the core's string slice type), which is a growable, mutable, owned, UTF-8 encoded string type, stored in the heap.  


* example 

```rust
let string_from_literal_1: &str = "Hello world";
let string_from_literal_2: String = "Hello world for std library's String";
```

### Rust core vs standard-library:
The Rust core library is the minimal, platform-agnostic foundation of the language, while the `std` library is the full standard library that builds upon core and adds platform-dependent capabilities like I/O and networking.

The core is not aware of features like heap allocation, concurrency, or file I/O, as these depends on OS.


### Rust namespace: 
A namespace is a logical grouping of declared names. Namespaces allow the occurrence of a name in one namespace to not conflict with the same name in another namespace. In layman's terms, namespaces are used to organize code and prevent naming conflicts.

```rust
// ---------------------- math.rs file ------------------------------
// Functions are public so they can be accessed from outside the module
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}


// ------------------------- main.rs file --------------------------
mod math; // Declares the module from the file

use math::add; // Bring only the 'add' function into scope
// use math::* // Bring all public items into scope

fn main() {
    let sum = add(5, 3); // Can now call 'add' directly
    let difference = math::subtract(5, 3); // Still need to qualify 'subtract'
    println!("Sum: {}, Difference: {}", sum, difference);
}
```

- `mod` : Declares a new module, creating a new namespace.
- `pub` : Makes items (functions, structs, etc.) public, allowing them to be accessed from outside their current module.
- `use` : Imports items into the current scope, so you don't have to use their full path every time.
- `::` : The namespace separator, used to access items within a module (e.g., std::collections::HashMap). 

### Ownership (heap) | coping is actually moving the ownership:
A String of made of 3 parts (std String)
- a pointer to the stack memory, that holds the content of the String (data)
- a length (actual) of the String
- a capacity, the total amount of memory in bytes that the allocator has allocated for the String  

When a heap-stored data, ie, `String` type of the standard library, is copied into another variable, (now) the new variable own the data, not the old variable. Its the heap stored data (pointer, length, capacity), not the stack stored actual data. 

* double free error: when two variables are pointing to the same data (copied to variable-2 from variable-1), when this 2 variables go out-of-scope, they will both try to free the same memory. Rust solve this by only making the later variable valid (owner)

```rust
// this will not work
let s1 = String::from("hello");
let s2 = s1;

// this will not work, as after copied, only s2 is valid. s1 is not valid
println!("{s1}, world!");
```

* shallow vs deep copy: copying the pointer, length, and capacity without copying the data is shallow copy. And coping everything is a deep copy. But rust's behavior is more like a `move`, where the shallow data is transferred/moved to a new variable and the old variable become obsolete immediately.

* Rust will never automatically create “deep” copies of your data. Shallow copy will always invalidate the old variable. To do deep copy, use `clone` method.

For mutable variables, when a new value is assigned to a mutable variable, Rust will run the drop function and the old value will be freed up immediately. (guess, the allocator will write new data to the stack and modify the mutable variable with new pointer, length and capacity)


```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // deep copy 

println!("s1 = {s1}, s2 = {s2}"); // valid 
```

### Stack only Data: Copy trait:
Types that have known size at compile time are stored entirely on the stack, as stack stored data is quick to make, coping data to a new variable perform an actual copy (not move like heap stored type). After coping, the old variable doesn't get invalid. Also there is no difference between shallow and deep copy in this case, all are same, also calling `clone` method will not do any special that the default copy.

```rust
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
```

* copy trait: Usually all stack-stored data types already comes with `copy` trait implemented. So they can be copied rather than move. Custom type can also implement `copy` trait, unless the type or parts of it implement `drop` trait (heap stored move).


* copy trait implemented types

- All the integer types, such as u32.
- The Boolean type, bool, with values true and false.
- All the floating-point types, such as f64.
- The character type, char.
- Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

### Ownership and function (without borrowing `&`):
Same ownership rules apply for function. If heap-stored type is passed as function's parameter, the ownership changes, and the previous variable will not be valid afterwards. Unless returned and captured by another variable, the results will get lost (as rust runtime will executed drop method for the previous variable)

```rust
const GLOBAL_CONSTANT: f64 = 3.1416;

pub fn ownership_and_function() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s); // s's value moves into the function...
    // println!("s = {s}"); // compile error, s is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // Because i32 implements the Copy trait,
    // x does NOT move into the function,
    // so it's okay to use x afterward.
    println!("As x is a stack-stored type it's stilled valid, x = {x}");


    println!("GLOBAL_CONSTANT is {GLOBAL_CONSTANT}"); // will work

} // Here, x goes out of scope, then s. However, because s's value was moved,
// nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope
```

### Capturing ownership moved variable with function's `return`:
Function can return value or values as tuple

```rust
#[allow(unused)]

pub fn return_value_and_variable_scope() {
    let s1 = gives_ownership(); // gives_ownership moves its return value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
    // takes_and_gives_back, which also
    // moves its return value into s3

    println!("s3 = {s3}"); // prints "s3 = hello world!"
    println!("{s1} {s3}"); // prints "yours hello world!"
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String { // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String, concatenate and returns the new String.
fn takes_and_gives_back(mut a_string: String) -> String {
    // a_string comes into scope

    a_string.push_str(" world!"); // pushing new worlds
    a_string  // a_string is returned and moves out to the calling function
}
```

* tuple return

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

### Reference and Borrowing | Retain ownership of the original variable:
Instead of passing a owner as a function parameter, we can pass a reference using `&`. This way we can retain the original owner and at the same time utilize the value without messing with ownership. In rust, the act of creating a reference is called `Borrowing`.

```rust
pub fn reference_and_borrowing() {
    let s1 = String::from("Hello World!");
    let s_len = calculate_length(&s1);
    println!("s1 = \"{s1}\" and length is sLen = {s_len}");
    // s1 is still valid as it had been borrowed using `&` and not used directly
}


fn calculate_length(str: &String) -> usize {
    str.len()
}
```

* an immutable borrowed prop/s cannot be mutated. But an immutable owned prop can be mutated using the `mut` keyword in the function's prop definition.

* `mutable reference` signature: `&mut T` for parameter and `&mut property` for argument/calling.  

```rust
pub fn borrowing_and_mutation() {
    let mut s1 = String::from("Hello");
    let s2 = change(&mut s1); // `&mut argument` for mutable borrowed reference 
    // if s1 is not mutable, then mutable borrowing will not work and throw `cannot borrow `s1` as mutable, as it is not declared as mutable`
    println!("s1 = {s1} and s2 = {s2}");
}

// signature for mutable reference (mutable borrowing) is `(param: &mut Type)`
// note: mutable reference will only work if the passed down variable/argument itself is mutable
fn change(str: &mut String) -> String {
    str.push_str(" World!");
    str.to_string()
}
```

* No Multiple Mutable Reference Restriction (prevent data race): A value can have only one mutable reference. More than one (in the same scope, or last used) will cause compile error `cannot borrow <variable> as mutable more than once at a time`

```rust
let mut s = String::from("hello");

let r1 = &mut s;
// let r2 = &mut s; // will throw compile error, cannot be borrowed as mutable more than noce

// but creating separate scope with just `{}` curly braces is fine for this case
let mut x = String::from("World!");

{
    let r3 = &mut x;
    println!("r1 = {r1}");

} // r1 goes out-of-scope and dropped, so the mutable reference is not active now

let r4 = &mut x;
```

* No combining of immutable and mutable Reference/Borrow: Unless scoped, it will also throw compile error


```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

println!("{r1}, {r2}, and {r3}"); // compile error
```

* Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used. 

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{r1} and {r2}");
// Variables r1 and r2 will not be used after this point.

let r3 = &mut s; // no problem
println!("{r3}");
```

### Dangling Reference:
When a pointer reference a location in memory that may have been given to someone else, and the pointer went to out-of-scope, is called Dangling pointer. Rust compiler guarantees, a reference will never be a dangling reference.

```rust
// this code will not compile
fn main() {
    let reference_to_nothing = dangle();
}

// this code will not compile because of dangling reference (trying to access deallocated/dropped memory)
fn dangle() -> &String {
    let s = String::from("Hello");
    &s // returning a reference to the s String
} // but here, s goes out of scope and is dropped, so it's memory goes away as well. Danger!
// so this function will not compile

// solution: don't return a reference in this case
// This works without any problems. Ownership is moved out, and nothing is deallocated
fn no_dangle() -> String {
    let s = String::from("hello");
    sg
}
```

* Rules of reference

    - At any given time, you can have either one mutable reference or any number of immutable references.
    - References must always be valid.

### Dereference type | `*T/V`:
a borrowed type can be dereferenced using `*`, but without the `&` will simply work

### Words collection without slice (first written programme):
```rust
main() {
    let letter = String::from("Hello World Again");
    let word_col = slicing_word_manual(&letter);
    println!("words = {:?}", word_col);
}

fn slicing_word_manual(words: &String) -> Vec<String> {
    // variable vector (container) with all the words
    let mut word_vector: Vec<String> = vec![];
    let mut a_word = String::from("");
    let bytes = words.as_bytes();

    // current pointer is the loop
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' || index == bytes.len() - 1 {
            if index == bytes.len() -1 {
                a_word.push(*&item as char);
            }
            word_vector.push(a_word.clone());
            a_word.clear();
            continue;
        }
        a_word.push(*&item as char); // `a_word.push(item as char);` will also work, plain and simple 
    }
    return word_vector;
}
```

### Slice type:
A slice is kind of a reference, so it doesn't have ownership. Slice lets us reference a contiguous sequence of elements is a collection.

* In idiomatic Rust, functions do not take ownership of their arguments unless they need to, ie `fn x(arg1: &T){}`, (and the reasons for that will become clear as we keep going)

### &String vs &str:
`&str` represents 2 types, `slice` (String sliced reference) and `string literal`. `&String` is the reference of a `String` (borrowed). 

Another thing is, `&str` is part of a `&String` but with limited method. Like `&str` slice type cannot be used to concatenate. 

* in function, it is possible to send `&String` as param and return a `&str`.

```rust
main() {
    let s = String::from("Hello World!");
    let full = full_string(&s);
    println!("full string is = {full}"); // prints "Hello World!"
}

fn full_string(x: &String) -> &str {
    x
}
```

### Destructuring a borrowed tuple or tuple element/s:

* Implicit borrowing (Match ergonomics):

```rust
fn main() {
    let data = ("foo".to_string(), "bar".to_string());
    let borrowed_data = &data;

    // The variables 'x' and 'y' are implicitly of type &String
    let (x, y) = borrowed_data; 

    println!("x: {}, y: {}", x, y); 
    // The original 'data' can still be used later
}
```

* Explicit Reference Patterns

```rust
fn main() {
    let data = (1, 2);
    let borrowed_data = &data;

    // The '&' in the pattern matches the reference from the right-hand side
    let (&x, &y) = borrowed_data; 

    // 'x' and 'y' are of type i32 (a copy, since i32 is Copy)
    println!("x: {}, y: {}", x, y);
}
```

* Using ref and ref mut

```rust
fn main() {
    let mut data = (1, 2);
    let borrowed_mut_data = &mut data;

    // Use 'ref mut' to get mutable references to the elements
    let (ref mut x, ref mut y) = *borrowed_mut_data;

    *x = 10;
    *y = 20;

    println!("data: {:?}", data); // Output: data: (10, 20)
}
```