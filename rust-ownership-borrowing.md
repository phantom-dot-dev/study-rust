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
{                      // s is not valid here, since it's not yet declared
    let s = "hello";   // s is valid from this point forward
    let t = "world";   // t is valid from here
}                      // this scope is now over, and s and t are no longer valid, t removed first, then the s had been removed. As of `stack`, last-in-first-out
```
