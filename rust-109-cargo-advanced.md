### Cargo Profiles:
Cargo has 2 main profile
- `cargo run` for the development profile
- `cargo run --release` for the release profile

Cargo has default settings for each of these, and be overridden using `[profile.dev/release]` section in Cargo.toml


* The code below in Cargo.toml file, will set explicit settings for dev and release build. opt-level value can be 0 to 3, 0 being the less optimized build taking least time for compiling

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

### Doc (Documentation comment):
Doc comments start with `///`, it supports markdown syntax and will be converted to `HTML` document when compiled with `cargo doc` command.

* For convenience, running `cargo doc --open` will build the HTML for the current crate’s documentation (as well as the documentation for all crate’s dependencies) and open the result in a web browser.


```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

* Note: Any code example written inside of a doc comment block will be tested as well by running `cargo test` or `cargo test --doc` (how cool is that).

```rust
/// ```
/// # use std::collections::HashMap;
/// // The line above is hidden from HTML documentation, but runs during tests!
/// let mut map = HashMap::new();
/// map.insert("status", "working");
/// ```
```


### Comments for the whole Item/Create `//!`:
This `//!` commenting are used to document the Create/Item as a whole. No block of code (fn, struct, etc) doesn't tied/coupled with this comment, rather these are usually place in the `Crate root` (src/lib.rs) 

* When we run cargo doc --open, these comments will display on the front page of the documentation for my_crate above the list of public items in the crate

```rust
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
// --snip--
```


### `pub use` to Re-export:
To provide clear guide on the doc page under the caret name (when `cargo doc --open` is run), public blocks of code can be re-exported by `pub use`. By this, the reader/user of the package will have full visible clue for all the available code blocks. 

And a public code block is nested deep inside physically, it can be re-exported for easy access. Like accessing by `use my_crate::UsefulType;` rather than drilling deep by `use my_crate::some_module::another_module::UsefulType;`



```rust
//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
    }
}


// ---------------Usages---------------
// use art::kinds::PrimaryColor;
// use art::utils::mix;

use art::PrimaryColor;
use art::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}

```

### Cargo Workspace:

Usually, when a project grows, the library crate continues to get bigger (as binary crate is for hosting small starting part of the program).

To split the library crate code, rust provide `workspace` feature to manage multiple related package (you cannot have more than one library crate along with binary crate without using `workspace` feature).

* A workspace is a set of package that share the same Cargo.lock and output directory. Through `workspace` we can have multiple binary and library crates in the same project.


A workspace definition starts in `Cargo.toml` file by stating `[workspace]` section (instead of `[package]` section). Followed by the `resolver` version (1 to 3, 3 being latest) for setting resolver algorithm.


* The Cargo resolver algorithm is a backtracking search system used by Rust's package manager, Cargo, to find and select compatible versions of software libraries (crates) for a project.

```toml
[workspace]
resolver = "3"
```

After this, adding a binary crate through cargo will populate `member` entry. IE, after running `cargo new adder` will add a new directory in the root as `adder` with the package (binary) specific `Cargo.toml` and `src/main.rs` inside.


* After running `cargo new adder`

```toml
[workspace]
resolver = "3"
members = ["adder"]
```

After adding one library crate by `cargo new add_one --lib`, cargo will add that like `members = ["adder", "add_one"]`

* Build Directory: through the workspace, there is only one target directory. Even-if we build package from inside a specific package/crate, the target directory will be the same, unified one, sitting in the root of the workspace.

* Cargo doesn’t assume that crates in a workspace will depend on each other, so we need to be explicit about the dependency relationships.

* Dependency Specification: To use any code in library crate/s from the binary crate, we have to specify the library crate name and it's path to the binary crate's Cargo.toml file under `[dependency]` section as (this can also be done using `cargo add add_one`)

```toml
<!-- Cargo.toml file inside of the binary crate -->\
[dependency]
add_one = { path = "../add_one }
```

After the inclusion in the binary crate's `Cargo.toml` 's dependency, we can use codes from the library crate/s.

```rust
fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
}
```

* `cargo run -p <binary_crate_name>` will run the specific crate


### Depending on external package/s while on workspace:
In a cargo workspace environment, we will have a global `Cargo.toml` file that will list all our local crates (all local binaries and libraries crates). And each local crate will also host their own local `Cargo.toml` file.

To use external crates, each local crate needs to specify that on their own `Cargo.toml` file's `[dependency]` section. 

* Adding external package: 

- Can be done manually or using 

- `Cargo add <crate_name>` command for non-workspace environment

- `cargo add <external_package> -p <local_crate_name>` command for cargo workspace environment, where we usually have multiple local crates.


### Version Conflict for External Crates:

Version Conflict Resolution (Same local crate but different version of a same external crate): 
Cargo will try to unify semver-compatible versions (Semantic Versioning: SemVar), but will compile multiple distinct versions if they are incompatible or strictly pinned. If not possible, Cargo will throw error


Version Conflict Resolution (Different local crate and different version of a same external crate):
- Cargo will compile and include both versions of the crate in the final build if their version requirements are semver-incompatible (Semantic Versioning: SemVer)

### Testing with workspace and little about publishing:
Test can be run same way, by `cargo test`, all test codes hosted through the entire workspace (all containing crates) will be run.

By `cargo test -p <local_crate_name>` will run test codes only hosted inside the specified crate.

For publishing, all available options can be found on the `https://doc.rust-lang.org/cargo/commands/cargo-publish.html`

From workspace, we can publish a single crate by `cargo publish -p <local_crate_name>` 

or all the crates/packages in the workspace by `cargo publish --workspace` 

or publish all but excluding some by `cargo publish --workspace --exclude package_a --exclude package_b`


```rust
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
```
