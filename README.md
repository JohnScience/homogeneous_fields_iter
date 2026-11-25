# homogeneous-fields

A Rust workspace for iterating over a struct's homogeneous fields. The
`HomogeneousFields` derive provides owned, shared, and mutable iterators. Each
iterator item is a pair of the field name (a `&'static str`) and the field
value or reference.

```rust
use homogeneous_fields::HomogeneousFields;

#[derive(HomogeneousFields)]
struct Counters {
    reads: u32,
    writes: u32,
}

let mut counters = Counters { reads: 3, writes: 4 };
for (name, value) in counters.fields() {
    println!("{name}: {value}");
}
for (_, value) in counters.fields_mut() {
    *value += 1;
}
let values: Vec<_> = counters.into_fields().collect();
```

Named fields use their identifiers as names. Tuple fields use their zero-based
indices as names. The derive requires at least one field, and all fields must
have the same type.

Run the workspace tests with `cargo test --workspace`, or try the usage example
with `cargo run -p homogeneous-fields-example`.
