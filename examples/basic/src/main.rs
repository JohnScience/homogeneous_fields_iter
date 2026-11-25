use homogeneous_fields::HomogeneousFields;

#[derive(HomogeneousFields)]
struct Scores {
    red: u32,
    blue: u32,
    green: u32,
}

fn main() {
    let mut scores = Scores {
        red: 3,
        blue: 5,
        green: 8,
    };

    for (name, score) in scores.fields() {
        println!("{name}: {score}");
    }

    for (_, score) in scores.fields_mut() {
        *score += 1;
    }

    let totals: Vec<_> = scores.into_fields().collect();
    assert_eq!(totals, [("red", 4), ("blue", 6), ("green", 9)]);
}
