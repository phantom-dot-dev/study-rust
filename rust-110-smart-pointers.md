### Smart Pointer:
A pointer is a general concept for a variable that contains an address in memory. For rust, these are indicated by `&` and borrow the value they point to.

Smart pointers, on the other hand, are data structures that act like a pointer but also have additional metadata and capabilities. Rust has a variety of smart pointers defined in the standard library that provide functionality beyond the capabilities of pointer/borrowing.

With rust, references/pointers borrow the data, but in many cases smart pointers own the data they point to.


Smart pointers are usually implemented using structs and implementing the `Deref` and `Drop` traits.


`Deref` Trait: to customize the behavior of the dereference operator (*), enabling smart pointers and wrapper types to be treated like regular references. IE, writing `*x` on a custom type that implements deref trail, will run `*(x.deref())` automatically.


`Drop` trait: to customize the code that’s run when an instance of the smart pointer goes out of scope. 

Common Smart pointers in rust

- `Box<T>`, for allocating values on the heap
- `Rc<T>`, a reference counting type that enables multiple ownership
- `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces the borrowing rules at runtime instead of compile time


### `Box<T>` Smart pointer:
It's the most straightforward smart pointer. It allows to store data on the heap (instead of stack) and the pointer in the stack (memory address).

`Box<T>` is useful when
- a type size is unknown at compile time, and the context is requiring an exact size
- When you have a large amount of data, and you want to transfer ownership but ensure that the data won’t be copied when you do so
- When you want to own a value, and you care only that it’s a type that implements a particular trait rather than being of a specific type

* Boxes provide only the indirection and heap allocation; they don’t have any other special capabilities. Using Box with recursive type provide rust compiler the exact required data size prediction, as with Box<T>, it's only the memory pointer storage (same for everything) and the actual data will be stored on the heap


