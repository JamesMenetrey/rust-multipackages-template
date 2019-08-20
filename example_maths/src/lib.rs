/// Multiplies two given number, represented as 64-bit integer.
///
/// ```
/// # use example_maths::multiply;
/// assert_eq!(multiply(9, 3), 27);
/// ```
pub fn multiply(x: u32, y: u32) -> u32 {
    x * y
}

// Inline unit testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(multiply(3, 3), 9);
    }
}
