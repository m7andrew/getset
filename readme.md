
# Derive Getters, Setters, and Builders

A **minimal** and opinionated Rust library for deriving getters, setters, and builders.

## Installation

To use in your project, add `getset` as a dependency to your `Cargo.toml` file:

```toml
[dependencies]
getset = { git = "https://github.com/m7andrew/getset" }
```

## Derive Getters & Setters

Getters and setters are useful when you want to control the API of a struct without exposing its fields directly. To keep things simple, the `GetSet` derive macro uses only two attributes: `#[get]` and `#[set]`.

```rust
#[derive(Debug, GetSet)]
pub struct Movie {
  #[get] title:   String,
  #[set] year:    u32,
  #[set] runtime: u32,
  #[set] genres:  Vec<String>,
}
```

- `#[set]` derives both a getter and setter function for the associated field.
- `#[get]` derives only a getter function for the associated field. 

By default, a field without either attribute derives nothing.

```rust
movie.year()         // Get
movie.set_year(2004) // Set
```

Setter functions can also be chained together:

```rust
movie
  .set_year(2004)
  .set_runtime(126)
```

If you need a setter without a getter, then it's best to implement the function manually. Typically in this situation, some amount of custom logic or validation is needed.

## Derive Builder

The builder pattern is not a replacement for named arguments, but rather, it's a useful pattern when you need to construct a type that possesses several optional or default values.

```rust
#[derive(Debug, Default, Builder)]
pub struct Movie {
  title:   String,
  year:    u32,
  runtime: u32,
  genres:  Vec<String>,
}
```

The `Builder` macro derives a builder type that wraps the original. The name of the builder type is the name of the original appended by "Builder".

Unlike many other builder libraries, the implementation of one or more constructors is left to you. This provides flexibility, letting you write constructors that require certain inputs or preform custom logic.

That said, you can often just wrap the default constructor:

```rust
impl Movie {
  pub fn new() -> MovieBuilder {
    MovieBuilder(Self::default())
  }
}
```
```rust
let movie = Movie::new()
  .title   ("Lord of the Rings".into())
  .year    (2001)
  .runtime (228)
  .build   ();
```

## Customize Your Builder

If needed, you can implement custom functions on the builder itself. Take the following example:

```rust
#[derive(Debug, Default, Builder)]
pub struct Rect {
  x:      i32,
  y:      i32,
  width:  i32,
  height: i32,
}

impl Rect {
  pub fn new(width: i32, height: i32) -> RectBuilder {
    RectBuilder(Self { width, height, ..Default::default() })
  }
}

impl RectBuilder {
  pub fn position(mut self, x: i32, y: i32) -> Self {
    self.0.x = x;
    self.0.y = y;
    self
  }
}
```
```rust
let shape = Rect::new(12, 12)
  .position(1, 3)
  .build();
```
