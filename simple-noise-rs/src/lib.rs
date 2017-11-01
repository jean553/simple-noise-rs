#[warn(dead_code)]

extern crate rand;
extern crate libc;

mod map;

/// Function to be called from C code, returns the generated map.
///
/// # Args:
///
/// `nodes` - C pointer to `int` array representing the map
/// `width` - the map width
/// `height` - the map height
/// `minimum` - the noise minimum value for a node
/// `maximum` - the noise maximum value for a node
#[no_mangle]
pub fn get_map(
    nodes: *const libc::int8_t,
    width: usize,
    height: usize,
    minimum: i8,
    maximum: i8,
) {
    let map = map::generate_map(
        width,
        height,
        minimum,
        maximum,
    );

    let nodes: &mut [i8] = unsafe {
        std::slice::from_raw_parts_mut(
            nodes as *mut i8,
            (width * height) as usize,
        )
    };

    for (counter, item) in map.iter().enumerate() {
        nodes[counter] = *item;
    }
}

#[cfg(test)]
mod tests; 
