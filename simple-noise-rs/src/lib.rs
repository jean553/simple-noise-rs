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
