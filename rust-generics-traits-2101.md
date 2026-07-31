### Generics with Enum:
`Option<T>` and `Result<T, E>` are example of generic Enum type

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### Method Definition (for Structs and Enums) in Generic way:
Structs and Enums can have their method/s defined using generics pattern as well

```rust
struct Point<T> {
    x: T,
    y: T,
}

// though seems redundant, By declaring T as a generic type after impl, Rust can identify that the type in the angle brackets in Point is a generic type rather than a concrete type. 
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point{x: 7, y: 7};
    println!("p.x = {}", p.x());
}
```


* We can also specify constraints on generic types when defining methods on the type

```rust
// defining constraints for implementation, the f32 will be the concrete type implementation 
impl Point<f32> {
    fn distance_form_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

```

### Traits in parameter and bounds:

```rust
// traits in function parameter, with shorthand bounding with `impl trait_name` syntax
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// traits with bound (explicit syntax), note: explicit syntax use generics 
pub fn notify<T: Summarize>(item: &T) {...}

// traits with explicit bound syntax can be less verbose for multiple parameters
pub fn notify(item1: &impl Summarize, item2: &impl Summarize){...}
pub fn notify<T: Summarize>(item1: &T, item2: &T){...}
```

### Multiple Trait Bounds using `+` operator (Trait + Trait) and use of `where`:
Using `+` or `where`, multiple trait bounds can be done.

```rust
// using + operator to enforce multiple trait bound, both syntactic sugar and explicit generic syntax
pub fn notify(item: &(impl Summary + Display)){...} // syntactic sugar or implicit syntax
pub fn notify<T: Summary + Display>(item: &T){...} // generic explicit syntax
```

`where` clause can be used to make multiple trait bound more readable. 
* As sometimes functions with multiple generic type parameters can contain lots of trait bound information between the function’s name and its parameter list, making the function signature hard to read

```rust
// example of little complex trait bound without using `where` clause
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {...}
// using where clause to bound multiple traits
fn some_function<T, U>(t: &T, u: &U) -> i32
where 
    T: Display + Clone,
    U: Clone + Debug,
{...}
```


### Returning type that implement the specified trait (same `impl Trait_name` syntax) and Restrictions:

Using `-> impl Trait_name` in the return position of a function, we can enforce to a value of some type that implements the specified trait

Note: Closures & Iterator benefits vastly through this (traits as return type)

```rust
fn return_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    }
}
```

* Restriction: a single type can be returned, conditional rerun of different value (of a trait type) can not be returned using this syntax (even if both value has implementation for the specified trait)

```rust
// note: return different type conditionally will not work here
// this code will not compile
fn this_fn_will_not_work(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {...}
    } else {
        SocialPost {...}
    }
}

// to return different values of same trait, use `Trait Object` as return type
// here Box<dyn Summary> is a Trait object, which is a heap allocated box implementation of the `dyn Summary`
fn returns_summarizable(switch: bool) -> Box<dyn Summary> {
    if switch {
        Box::new(NewsArticle {...})
    } else {
        Box::new(SocialPost {...})
    }
}
```

### Trait bounds to conditionally implement methods:
By using a trait bound with an impl block that uses generic type parameters, methods can be implemented conditionally for types that implement the specified traits. 

In the code below, through `impl<T: Display + PartialOrd> Pair<T> {...}`, we super charge the generic type `T` bounding with Display and PartialOrd trait, Display trait helps to print the value and PartialOrd trait helps to compare values

* `self` vs `Self`: in rust `self` refers to a instance of the struct or enum within it's definition scope, and `Self` refers to the actual type


```rust
use std::fmt::Display;

pub fn trait_with_conditional_method_impl() {
    println!("\n\n------------Trait with conditional method implementation----------------\n\n");

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
```


### Blanket Implementation of trait (conditional trait implementation):
Blanket Implementation is the term used when a trait implementation (on a concrete or generic type) is done conditionally based on another trait.

```rust
// here the ToString trait gets available for any type that implements Display trait
// this code snippet is from the standard library.
impl<T: Display> ToString for T {...}
```


* Because of this blanket implementation, we can call to_string method (defined by `ToString` trait) on any type that implement the Display trait.

```rust
// because, integers type implements Display trait, we can get it's string value
// by calling integer.to_string() method (through ToString trait of-course)
let numbers_to_string = 7.to_string();
```

### Lifetimes and Reference Validation:
Lifetimes are a kind of generics that ensures "reference to any variable is valid as long as we need them". Lifetimes are most of the time implicit (we don't need to write to specify them every time), but sometimes we must annotate lifetimes to clear up any ambiguity.

```rust
// why the code will not compile
// passing no borrowed value in the parameter and returning a borrowed value at the same time will not compile, as it will create a dangling pointer
// when we're passing only one borrowed value in the parameter and returning 
// when we're passing more than one borrowed values in the parameter, 
fn longest_str(x: &str, y: &str) -> &str {...}
```


### Variable delayed initialization (doesn't need `mut` keyword):
Variable can be declared first and initialized later. 
The compiler uses an analysis mechanism called `definite assignment` to ensure the variable is assigned a value before can be read.


* Because of the safety check (definite assignment), the immutability can be maintained (not using the mut keyword) whiling delaying its initialization.

```rust
fn main() {
    let a: i32; // to use delayed initialization immutably, do without `mut` keyword 
    {
        let b = 7;
        // a = &b; won't work, as the value of will be removed from the memory as soon as the scope ends, hence rust compiler won't allow us to assign it's address (borrowed value = memory address) to a variable. This check is done by borrow checker (if all borrows are valid)
        a = b; // works and uses delayed initialization
    }

    // a = 77; // assigning a new value will not work unless `a` was declared mutable when declared
    println!("Hello form {a}")
}
```


### Lifetime Annotation Syntax:
Lifetime annotations don't change the lifetime, rather, it describe the relationships of the lifetimes of multiple references to each other.

Syntax: The names of lifetime parameters must start with an apostrophe (') and are usually all lowercase and very short. Place the annotation after `&` of a reference, use a space to separate the annotation from the reference's type

* lifetime annotations are used mostly with `Function Signature`, `Struct's Definition`, `Method Definitions`

```rust
&i32; // a reference without lifetime annotation
&'a i32; // a reference with a lifetime annotation `a`
&'a mut i32; // a mutable reference with a lifetime annotation `a`

// lifetime annotations are used with generic lifetime parameter inside angle brakes between the function name and the parameter/s
// the code below, we're  specifying that all the references in the signature must have the same lifetime 'a
// lifetime annotation in functions go in the function's signature, not in the body
fn longest<'a>(x: &'a str, y: &'a str) ->  &'a str {
    if x.len() > y.len() { x } else { y }
}
// the signature says (for lifetime annotation): The returned reference will be valid as long as both of the parameters are valid. So when any of this actual value/argument (concert reference) goes out of scope (removed). 

// In other words, the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and y. Because we’ve annotated the returned reference with the same lifetime parameter 'a, the returned reference will also be valid for the length of the smaller of the lifetimes of x and y 
```

* One lifetime annotation by itself doesn’t have much meaning, because the annotations are meant to tell Rust how generic lifetime parameters of multiple references relate to each other. Hence they are composed with lifetime markers in generics like brackets `fn name<'a, 'b, T>(x: &'a str, y: &'b str, z: T) -> &'a str {/*lifetime parameter never goes in side body*/}`

* Practical example of how lifetime annotations works by defining scoping context (describing relationship) to the rust compiler

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// this works
fn main() {
    let string1 = String::from("long string is long"); // this has the longest lifetime

    {
        let string2 = String::from("xyz"); // this has the shortest lifetime
        let result = longest(string1.as_str(), string2.as_str()); // `result` variable will have the same lifetime as `string2`,
        println!("The longest string is {result}");
        // note: this works as our printing is happening in the same scope as shortest
    }
}


// this doesn't work
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str()); // fails
        // it fails because, the string2 will be removed as the scope ends, memory will be cleaned hence reference pointing to that memory will be obsolete, creating no chance for dangling pointer
        // in other words, string2 would need to be valid until the end of the outer scope
        // as in longest function signature, lifetime of the reference returned by the longest function is the same as the smaller (string2) of the lifetimes of the references passed in.
        // ----------------------- working solution
        // to make it work, we have to convert the borrowed reference type into a owned type. Then the `result` is no longer holding just a reference, rather actual data
        // result = longest(string1.as_str(), string2.as_str()).to_string(); // works
    }
    println!("The longest string is {result}");
}
```

* Lifetime relationships with parameter/s and functions return value: If a parameter has no relationship with it the return value, we don't need to specify lifetime parameter for that parameter

```rust
// there's a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, because the lifetime of y does not have any relationship with the lifetime of x or the return value.
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

* Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions, not to change lifetime. Once they’re connected, Rust has enough information to allow memory-safe operations and disallow operations that would create dangling pointers or otherwise violate memory safety.


### Lifetime Annotation with Struct Definition:
To hold reference/borrowed type, we need to add a lifetime annotation on every reference type in the struct definition.

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let song = String::from("Caught Somewhere in time");
    let first_word = song.split(' ').next().unwrap();
    let i = ImportantExcerpt {
        part: first_word,
    }

    // novel doesn’t go out of scope until after the ImportantExcerpt goes out of scope, so the reference in the ImportantExcerpt instance is valid
}
```


### LIfetime Elision (patterns used in compiler to automatically determine lifetime):
The patterns programmed into Rust’s analysis of references are called the lifetime elision rules. These aren’t rules for programmers to follow; they’re a set of particular cases that the compiler will consider, and if your code fits these cases, you don’t need to write the lifetimes explicitly.

Because of this, after rust version 1.0, we don't need to write lifetime annotations for reference type that accepts and return only one parameter.

```rust
// before rust version 1.0, we have to write lifetime annotation even for a single parameter
fn first_word<'a>(s: &'a str) -> &'a str { s }


// after rust version 1.0, lifetime elision pattern can determine the relationship automatically, so we don't need to be explicit for single parameter (reference type) function
fn first_word(s: &str) -> &str { s }
```

The elision rules don’t provide full inference. If there is still ambiguity about what lifetimes the references have after Rust applies the rules, the compiler won’t guess what the lifetime of the remaining references should be. Instead of guessing, the compiler will give you an error that you can resolve by adding the lifetime annotations.

* Lifetime elision rules (3) fro functions and methods
1st - the compiler assigns a lifetime parameter to each parameter that’s a reference

2nd - if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32

3rd - The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters.

When a function definition doesn't fall into this rules, we have to manually assign the lifetimes to make it clear for the rust compiler to resolve disambiguate so that no dangling pointer can be exists.


### Lifetime annotation with Method Definitions:
Lifetime names for struct fields always need to be declared after the impl keyword and then used after the struct’s name because those lifetimes are part of the struct’s type.

```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        7
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}
```


### Static Lifetime:
All string literal have the 'static' lifetime, which live for the entire duration of the program.

```rust
let s: &'static str = "I have a static lifetime.";
// The text of this string is stored directly in the program’s binary, which is always available. Therefore, the lifetime of all string literals is 'static.
```

* when an error message suggesting the 'static lifetime results from attempting to create a dangling reference or a mismatch of the available lifetimes. Sometime the solution is to fix those problems, not to specify the 'static lifetime.

### Generic Type Parameters, Trait Bounds & Lifetimes:
Lifetimes are a type of generic, the declarations of the lifetime parameter 'a and the generic type parameter T go in the same list inside the angle brackets after the function name.

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T> (
    x: &'a str,
    y: &'a str,
    ann: T
) -> &'a str
where 
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}
```
