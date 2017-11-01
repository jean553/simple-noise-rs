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
) -> Vec<u8> {

    let map = vec![0; width * height];

    // TODO: added here just for the interface, must be defined
    map
}
