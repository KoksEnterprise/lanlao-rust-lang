// Utility functions for conversions

/// Converts an integer to a string.
///
/// # Examples
///
/// ```
/// let num = 42;
/// let result = int_to_string(num);
/// assert_eq!(result, "42");
/// ```
pub fn int_to_string(num: i32) -> String {
    num.to_string()
}
