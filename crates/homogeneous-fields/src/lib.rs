pub use homogeneous_fields_derive::HomogeneousFields;

/// Iterates over the fields of a struct when all of its fields have one type.
///
/// The derive macro implements this trait for non-empty structs. Named fields
/// are identified by their names, and tuple fields by their zero-based indices.
pub trait HomogeneousFields {
    /// The common type of the struct's fields.
    type Item;

    /// The iterator returned by [`into_fields`](Self::into_fields).
    type IntoIter: Iterator<Item = (&'static str, Self::Item)>;

    /// The iterator returned by [`fields`](Self::fields).
    type Iter<'a>: Iterator<Item = (&'static str, &'a Self::Item)>
    where
        Self: 'a;

    /// The iterator returned by [`fields_mut`](Self::fields_mut).
    type IterMut<'a>: Iterator<Item = (&'static str, &'a mut Self::Item)>
    where
        Self: 'a;

    /// Consumes the struct and yields each field name and value.
    fn into_fields(self) -> Self::IntoIter;

    /// Borrows the struct and yields each field name and value.
    fn fields(&self) -> Self::Iter<'_>;

    /// Mutably borrows the struct and yields each field name and value.
    fn fields_mut(&mut self) -> Self::IterMut<'_>;
}
