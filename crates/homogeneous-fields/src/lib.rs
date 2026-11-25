//! Iterate over a struct's homogeneous fields by value, shared reference, or
//! mutable reference.
//!
//! Derive [`HomogeneousFields`] on a non-empty struct whose fields all have the
//! same type. Each iterator item is a pair containing a static field name and
//! the field value (or a reference to it). Named fields use their identifiers;
//! tuple fields use their zero-based indices converted to strings.
//!
//! ```
//! use homogeneous_fields::HomogeneousFields;
//!
//! #[derive(HomogeneousFields)]
//! struct Counters {
//!     reads: u32,
//!     writes: u32,
//! }
//!
//! let mut counters = Counters { reads: 3, writes: 4 };
//! let names_and_values: Vec<_> = counters.fields().map(|(name, value)| (name, *value)).collect();
//! assert_eq!(names_and_values, [("reads", 3), ("writes", 4)]);
//!
//! for (_, value) in counters.fields_mut() {
//!     *value += 1;
//! }
//!
//! let owned: Vec<_> = counters.into_fields().collect();
//! assert_eq!(owned, [("reads", 4), ("writes", 5)]);
//! ```
pub use homogeneous_fields_derive::HomogeneousFields;

/// Iterates over a struct's fields when all fields have the same type.
///
/// The derive macro implements this trait for non-empty structs. A struct with
/// fields of different types does not implement this trait. Enums and unit
/// structs are not supported by the derive.
pub trait HomogeneousFields {
    /// The common type of every field.
    type Item;

    /// The iterator yielded by [`into_fields`](Self::into_fields).
    type IntoIter: Iterator<Item = (&'static str, Self::Item)>;

    /// The iterator yielded by [`fields`](Self::fields).
    type Iter<'a>: Iterator<Item = (&'static str, &'a Self::Item)>
    where
        Self: 'a;

    /// The iterator yielded by [`fields_mut`](Self::fields_mut).
    type IterMut<'a>: Iterator<Item = (&'static str, &'a mut Self::Item)>
    where
        Self: 'a;

    /// Consumes the struct and yields each field's name and owned value.
    ///
    /// Field names are `&'static str`s. Named fields use their identifier;
    /// tuple fields use their zero-based index as a decimal string.
    fn into_fields(self) -> Self::IntoIter;

    /// Borrows the struct and yields each field's name and shared reference.
    ///
    /// The returned references borrow `self`.
    fn fields(&self) -> Self::Iter<'_>;

    /// Mutably borrows the struct and yields each field's name and mutable
    /// reference.
    ///
    /// The returned references mutably borrow `self`.
    fn fields_mut(&mut self) -> Self::IterMut<'_>;
}
