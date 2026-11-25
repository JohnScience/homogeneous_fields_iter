# `homogeneous-fields-derive`

This crate provides the `#[derive(HomogeneousFields)]` procedural macro used
by [`homogeneous-fields`](https://crates.io/crates/homogeneous-fields).
The macro generates implementations of the library's
[`HomogeneousFields` trait](https://docs.rs/homogeneous-fields/latest/homogeneous_fields/trait.HomogeneousFields.html),
including owned, shared, and mutable field iterators.

Most users should depend on `homogeneous-fields`, which re-exports this macro
and provides the trait:

```toml
[dependencies]
homogeneous-fields = "0.1.1"
```

```rust
use homogeneous_fields::HomogeneousFields;

#[derive(HomogeneousFields)]
struct Pair {
    left: i32,
    right: i32,
}

let pair = Pair { left: 10, right: 20 };
assert_eq!(
    pair.fields().map(|(name, value)| (name, *value)).collect::<Vec<_>>(),
    [("left", 10), ("right", 20)],
);
```

## Supported input

The derive accepts non-empty named-field and tuple structs whose fields all
have the same type. It supports generic structs. Generated names use named
field identifiers or the zero-based tuple field indices converted to decimal
strings. Iteration follows the order in which fields are declared.

Enums and unit structs are rejected. A struct whose fields have different
types fails to compile because the generated iterator requires a single common
item type.

## Direct use

The macro expansion refers to the `homogeneous_fields` crate path. Therefore,
when depending on this proc-macro crate directly, also add
`homogeneous-fields` as a dependency and import the derive macro from this
crate:

```toml
[dependencies]
homogeneous-fields = "0.1.1"
homogeneous-fields-derive = "0.1.1"
```

```rust
use homogeneous_fields::HomogeneousFields as HomogeneousFieldsTrait;
use homogeneous_fields_derive::HomogeneousFields;
```

Using `homogeneous-fields` alone is recommended; it re-exports the derive
macro under the same name as the trait, which Rust permits in separate
namespaces.

## Links

- [API documentation](https://docs.rs/homogeneous-fields-derive)
- [Main library crate](https://crates.io/crates/homogeneous-fields)
- [Source repository](https://github.com/JohnScience/homogeneous_fields_iter)
