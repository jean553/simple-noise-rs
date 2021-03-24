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
    let mut previous_line = vec![
        DEFAULT_NODE_VALUE;
        width
    ];
    for (index, node) in map.iter_mut().skip(1).enumerate() {

        const DIFFERENCE_MINIMUM: i8 = -1;
        const DIFFERENCE_MAXIMUM: i8 = 2;
        let difference = random_number_generator.gen_range(
            DIFFERENCE_MINIMUM,
            DIFFERENCE_MAXIMUM, // [-1;2[
        );

        let mut value = previous_value + difference;

        if index >= width {

            let previous_line_value = previous_line[index % width];

            value = if index % width != 0 {
                (previous_line_value + previous_value) / 2
            } else {
                previous_line_value
            };

            value += difference;
        }

        if value < minimum {
            value = minimum;
        }
        else if value > maximum {
            value = maximum;
        }

        *node = value;

        let previous_line_index = index % width;
        previous_line[previous_line_index] = value;

        previous_value = value;
    }

    map
}
