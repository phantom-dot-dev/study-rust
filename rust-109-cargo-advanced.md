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

