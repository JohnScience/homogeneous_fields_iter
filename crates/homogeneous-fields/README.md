# `homogeneous-fields`

`homogeneous-fields` provides a trait and derive macro for iterating over all
the fields of a struct when they share one type. Each iterator item is a pair:
`(field_name: &'static str, value)`.

The crate re-exports the `HomogeneousFields` derive macro, so a dependency on
this crate is sufficient for typical use:

```toml
[dependencies]
homogeneous-fields = "0.1.1"
```

## Example

```rust
use homogeneous_fields::HomogeneousFields;

#[derive(HomogeneousFields)]
struct Counters {
    reads: u32,
    writes: u32,
}

let mut counters = Counters { reads: 3, writes: 4 };

// Shared iteration yields (&'static str, &T).
for (name, value) in counters.fields() {
    println!("{name}: {value}");
}

// Mutable iteration yields (&'static str, &mut T).
for (_, value) in counters.fields_mut() {
    *value += 1;
}

// Owned iteration consumes the struct and yields (&'static str, T).
let fields: Vec<_> = counters.into_fields().collect();
assert_eq!(fields, [("reads", 4), ("writes", 5)]);
```

## Supported structs

- Every field must have the same type.
- The struct must have at least one field.
- Named fields are labeled with their exact identifiers, such as `"reads"`.
- Tuple fields are labeled with their zero-based indices as decimal strings,
  such as `"0"` and `"1"`.
- Generic structs are supported when all of their fields have the same type.
- Enums, unit structs, and structs with heterogeneous field types are not
  supported.

The generated iterators preserve declaration order and implement `Iterator`
using fixed-size arrays; no heap allocation is needed.

## API

The [`HomogeneousFields` trait](https://docs.rs/homogeneous-fields/latest/homogeneous_fields/trait.HomogeneousFields.html)
provides:

- `fields(&self)`: iterate over shared references without consuming the struct.
- `fields_mut(&mut self)`: iterate over mutable references without consuming
  the struct.
- `into_fields(self)`: consume the struct and iterate over owned values.

Each method returns a concrete iterator type through the trait's associated
types. The derive infers the common field type as `HomogeneousFields::Item`.

## Links

- [API documentation](https://docs.rs/homogeneous-fields)
- [Derive crate](https://crates.io/crates/homogeneous-fields-derive)
- [Source repository](https://github.com/JohnScience/homogeneous_fields_iter)
