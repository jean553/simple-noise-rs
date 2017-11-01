extern crate rand;

use rand::{
    Rng,
    thread_rng,
};

/// Public interface that generates a map according to the given dimensions.
///
/// # Args:
///
/// `width` - the map width
/// `height` - the map height
///
/// # Returns:
///
/// Vector with the map nodes values.
///
/// TODO: Should return a Vec<i8>, the option is needed for the map generation.
pub fn generate_map(
    width: usize,
    height: usize,
    minimum: i8,
    maximum: i8,
) -> Vec<Option<i8>> {

    let mut map = vec![
        None;
        width * height
    ];

    let mut random_number_generator = thread_rng();

    map[0] = Some(
        random_number_generator.gen_range(
            minimum,
            maximum,
        )
    );

    // TODO: added here just for the interface, must be defined
    map
}

#[cfg(test)]
mod tests {

    use generate_map;

    #[test]
    fn test_first_index_random_generation() {

        const WIDTH: usize = 10;
        const HEIGHT: usize = 10;
        const MINIMUM: i8 = -10;
        const MAXIMUM: i8 = 10;
        let values = generate_map(
            WIDTH,
            HEIGHT,
            MINIMUM,
            MAXIMUM,
        );

        let first_value = values[0].unwrap();

        assert!(
            first_value > -10,
            format!(
                "The first node {} value is too small.",
                first_value,
            )
        );

        assert!(
            first_value < 10,
            format!(
                "The first node {} value is too high.",
                first_value,
            )
        );
    }
}
