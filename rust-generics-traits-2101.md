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
// defining constraints for implementation
impl Point<f32> {
    fn distance_form_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

```
