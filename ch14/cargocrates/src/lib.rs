//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds two numbers together
///
/// # Examples
/// ```
/// let result = cargocrates::add(2, 2);
/// assert_eq!(result, 4);
/// ```
/// # Panics
/// ```should_panic
/// let result = cargocrates::add(2, 2);
/// assert_eq!(result, 5);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
