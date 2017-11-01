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
) -> Vec<i8> {

    const DEFAULT_NODE_VALUE: i8 = 0;
    let mut map = vec![
        DEFAULT_NODE_VALUE;
        width * height
    ];

    let mut random_number_generator = thread_rng();

    map[0] = random_number_generator.gen_range(
        minimum,
        maximum,
    );

    let mut previous_value = map[0];
    for node in map.iter_mut().skip(1) {

        const DIFFERENCE_MINIMUM: i8 = -1;
        const DIFFERENCE_MAXIMUM: i8 = 2;
        let difference = random_number_generator.gen_range(
            DIFFERENCE_MINIMUM,
            DIFFERENCE_MAXIMUM, // [-1;2[
        );

        let mut value = previous_value + difference;

        if value < minimum {
            value = minimum;
        }
        else if value > maximum {
            value = maximum;
        }

        *node = value;
        previous_value = value;
    }

    map
}
