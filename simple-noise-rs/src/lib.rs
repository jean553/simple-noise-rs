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
/// TODO: Should return a Vec<u8>, the option is needed for the map generation.
pub fn generate_map(
    width: usize,
    height: usize,
) -> Vec<Option<u8>> {

    let mut map = vec![
        None;
        width * height
    ];

    map[0] = Some(rand::random::<u8>());

    // TODO: added here just for the interface, must be defined
    map
}
