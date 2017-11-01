use map::generate_map;

#[test]
fn test_first_index_random_generation() {

    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;
    const MINIMUM: i8 = -3;
    const MAXIMUM: i8 = 3;
    let values = generate_map(
        WIDTH,
        HEIGHT,
        MINIMUM,
        MAXIMUM,
    );

    let first_value = values[0];

    assert!(
        first_value >= MINIMUM,
        format!(
            "The first node {} value is too small.",
            first_value,
        )
    );

    assert!(
        first_value < MAXIMUM,
        format!(
            "The first node {} value is too high.",
            first_value,
        )
    );
}

#[test]
fn test_first_line_nodes_generation() {

    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;
    const MINIMUM: i8 = -3;
    const MAXIMUM: i8 = 3;
    let values = generate_map(
        WIDTH,
        HEIGHT,
        MINIMUM,
        MAXIMUM,
    );

    for (index, value) in values.iter().enumerate() {

        let value = *value;

        assert!(
            value >= MINIMUM,
            format!(
                "The node {} value ({}) is too small.",
                index,
                value,
            )
        );

        assert!(
            value < MAXIMUM,
            format!(
                "The node {} value ({}) is too high.",
                index,
                value,
            )
        );
    }
}
