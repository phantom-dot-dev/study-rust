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
