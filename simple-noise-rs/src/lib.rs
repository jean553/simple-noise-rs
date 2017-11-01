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
/// TODO: Should return a Vec<u8>, the option is needed for the map generation.
pub fn generate_map(
    width: usize,
    height: usize,
) -> Vec<Option<u8>> {

    let map = vec![
        None;
        width * height
    ];

    let center_index = get_center_node_index(
        width,
        height,
    );

    // TODO: added here just for the interface, must be defined
    map
}

/// Returns the index of the center node of the map.
///
/// # Args:
///
/// `width` - the map width
/// `height` - the map height
///
/// # Returns:
///
/// The index of the center.
fn get_center_node_index(
    width: usize,
    height: usize,
) -> usize {

    let horizontal_index = width / 2;
    let vertical_index = height / 2;

    vertical_index * width + horizontal_index
}
