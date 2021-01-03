# gtmpl_derive &emsp; [![Latest Version]][crates.io]
[Latest Version]: https://img.shields.io/crates/v/gtmpl_derive.svg
[crates.io]: https://crates.io/crates/gtmpl_derive


**Derive macro for [gmtpl_value][gtmpl_value-github]**

---

```toml
[dependencies]
gtmpl_derive = "0.4"
```

* [gtmpl_derive at crates.io](https://crates.io/crate/gtmpl_derive)
* [gtmpl_derive documentation](https://docs.rs/crate/gtmpl_derive)

## Usage

```rust
use gtmpl_derive::Gtmpl;
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
