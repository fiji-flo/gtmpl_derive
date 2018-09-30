# gtmpl_derive &emsp; [![Build Status]][travis] [![Latest Version]][crates.io]
[Build Status]: https://travis-ci.org/fiji-flo/gtmpl_derive.svg?branch=master
[travis]: https://travis-ci.org/fiji-flo/gtmpl_derive
[Latest Version]: https://img.shields.io/crates/v/gtmpl_derive.svg
[crates.io]: https://crates.io/crates/gtmpl_derive


**Derive macro for [gmtpl_value][gtmpl_value-github]**

---

```toml
[dependencies]
gtmpl_derive = "0.3.3"
```

* [gtmpl_derive at crates.io](https://crates.io/crate/gtmpl_derive)
* [gtmpl_derive documentation](https://docs.rs/crate/gtmpl_derive)

## Usage

```rust
#[macro_use]
extern crate gtmpl_derive;
extern crate gtmpl_value;
use gtmpl_value::Value;

#[derive(Gtmpl)]
struct Foo {
    bar: u8
}

fn main() {
    let v: Value = (Foo { bar: 23 }).into();
}
```

[gtmpl_value-github]: https://github.com/fiji-flo/gtmpl_value
