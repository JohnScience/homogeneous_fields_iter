use homogeneous_fields::HomogeneousFields;

#[derive(HomogeneousFields)]
struct Named {
    first: i32,
    second: i32,
}

#[derive(HomogeneousFields)]
struct Tuple<T>(T, T);

#[test]
fn named_fields_support_all_iterator_modes() {
    let mut value = Named {
        first: 2,
        second: 5,
    };

    assert_eq!(
        value.fields().collect::<Vec<_>>(),
        [("first", &2), ("second", &5)]
    );

    for (_, field) in value.fields_mut() {
        *field *= 2;
    }
    assert_eq!(
        value.into_fields().collect::<Vec<_>>(),
        [("first", 4), ("second", 10)]
    );
}

#[test]
fn tuple_fields_use_indices_and_support_generics() {
    let value = Tuple(String::from("a"), String::from("b"));

    assert_eq!(
        value
            .fields()
            .map(|(name, field)| (name, field.as_str()))
            .collect::<Vec<_>>(),
        [("0", "a"), ("1", "b")]
    );
}
